#![feature(clamp)]
#![feature(core_intrinsics)]

#[macro_use]
extern crate lazy_static;

use std::intrinsics::ctlz;

pub(in crate) mod geometry;
pub use self::geometry::{intersects, BoundingBox, Intersection, RotatedRect};

#[derive(Copy, Clone)]
struct Node<T> {
    count: u16,
    item: Option<(u16, T)>,
}

pub struct Tree<T> {
    nodes: Vec<Node<T>>,
    pub width: f32,
    pub height: f32,
}

lazy_static! {
    static ref LEVELS: [u16; 8] = {
        let mut levels = [0; 8];
        let mut i: u16 = 0;
        for j in 1..8 {
            let k = j - 1;
            i += (1 << k) * (1 << k);
            levels[j as usize] = i;
        }

        levels
    };
}

trait BitScan {
    fn bsr(self) -> Self;
}

impl BitScan for u8 {
    fn bsr(self) -> Self {
        7 - ctlz(self)
    }
}

impl From<BoundingBox<f32>> for BoundingBox<u8> {
    fn from(item: BoundingBox<f32>) -> Self {
        let clamp = |x: f32| x.clamp(0.0, 255.0);
        BoundingBox {
            x0: clamp(item.x0.floor()) as u8,
            y0: clamp(item.y0.floor()) as u8,
            x1: clamp(item.x1.ceil()) as u8,
            y1: clamp(item.y1.ceil()) as u8,
        }
    }
}

impl<T> Tree<T>
where
    T: Clone,
{
    pub fn new(width: f32, height: f32) -> Tree<T> {
        let nodes: Vec<Node<T>> = Vec::with_capacity(5);

        Tree {
            nodes,
            width,
            height,
        }
    }

    pub fn insert_checked(
        &mut self,
        item: T,
        bbox: &BoundingBox<f32>,
        check: Option<&Fn(&T, &T) -> bool>,
    ) -> bool {
        let inv_w = 256.0 / self.width;
        let inv_h = 256.0 / self.height;

        // Clamp and convert bounds to byte range.
        let shifted: BoundingBox<u8> = BoundingBox::new(
            bbox.x0.max(0.0) * inv_w,
            bbox.y0.max(0.0) * inv_h,
            bbox.x1.min(self.width) * inv_w,
            bbox.y1.min(self.height) * inv_h,
        )
        .into();

        // Get the level at which the object will be inserted. We use
        // a bithack from Game Programming Gems II by Matt Pritchard,
        // courtesy of L. Spiro.
        let x = shifted.x0 ^ shifted.x1;
        let x = if x == 0 { 7 } else { 7 - x.bsr() };

        let y = shifted.y0 ^ shifted.y1;
        let y = if y == 0 { 7 } else { 7 - y.bsr() };

        let level = x.min(y);

        for i in (0..level + 1).rev() {
            let length = {
                let base = LEVELS[level as usize];
                let index = if i == 0 {
                    base
                } else {
                    let s = 1 << i;
                    let k: u32 = 8_u32 - i as u32;
                    let x = shifted.x0 as u8 >> k;
                    let y = shifted.y0 as u8 >> k;
                    base + y as u16 * s + x as u16
                };
                self.resize(index as usize)
            };

            let index = LEVELS[i as usize];
            let mut k: usize = index as usize;
            let count = self.nodes[k].count;
            let mut remaining = count;
            let mut slot = None;

            for _ in 0..length {
                match &self.nodes[k].item {
                    Some((other_index, other_item)) => {
                        if *other_index == index {
                            if let Some(f) = &check {
                                if f(&item, other_item) {
                                    return false;
                                }
                            }

                            remaining -= 1;
                        }
                    }
                    None => {
                        if slot.is_none() {
                            slot = Some(k);
                        }

                        if remaining == 0 {
                            break;
                        }

                        ()
                    }
                }

                // Wrap around
                k += 1;
                if k == length {
                    k = 0;
                }
            }

            if i == 0 || check.is_none() {
                let j = match slot {
                    Some(j) => j,
                    None => {
                        &self.nodes.push(Node {
                            count: 0,
                            item: None,
                        });
                        length
                    }
                };

                self.nodes[j].item = Some((index, item));
                self.nodes[index as usize].count = count + 1;

                return true;
            }
        }

        false
    }

    pub fn insert_unless_intersecting(
        &mut self,
        item: T,
        bbox: &BoundingBox<f32>,
    ) -> bool
    where
        T: Intersection,
    {
        self.insert_checked(item, bbox, Some(&intersects))
    }

    pub fn insert(&mut self, item: T, bbox: &BoundingBox<f32>) {
        self.insert_checked(item, bbox, None);
    }

    fn resize(&mut self, length: usize) -> usize {
        let cur_length = self.nodes.len();
        if cur_length > length {
            return cur_length;
        }

        let mut new_length = 0;
        for i in 0..8 {
            let term = 2_usize.pow(2 * i);
            new_length += term;
            if new_length > length {
                &self.nodes.resize(
                    new_length,
                    Node {
                        count: 0,
                        item: None,
                    },
                );
                break;
            }
        }

        return new_length;
    }
}
