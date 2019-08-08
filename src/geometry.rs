use std::ops::{Add, Neg, Sub};

#[derive(Clone, Copy, Debug)]
pub struct BoundingBox<T: Copy> {
    pub x0: T,
    pub y0: T,
    pub x1: T,
    pub y1: T,
}

impl<T> BoundingBox<T>
where
    T: Copy + PartialOrd + Neg<Output = T>,
{
    pub fn new(x0: T, y0: T, x1: T, y1: T) -> Self {
        BoundingBox { x0, y0, x1, y1 }
    }
}

pub trait Intersection {
    fn intersects(&self, other: &Self) -> bool;
}

#[derive(Debug, Clone, Copy)]
pub struct RotatedRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub a: f32,
}

#[derive(Debug, Clone, Copy)]
struct Vector(f32, f32);

impl Vector {
    fn rotate(&self, a: f32) -> Self {
        let cosa = a.cos();
        let sina = a.sin();
        Vector(
            self.0 * cosa + self.1 * sina,
            -self.0 * sina + self.1 * cosa,
        )
    }
}

impl Add<Vector> for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector(self.0 + other.0, self.1 + other.1)
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self {
        Vector(-self.0, -self.1)
    }
}

impl Sub<Vector> for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector(self.0 - other.0, self.1 - other.1)
    }
}

impl From<&RotatedRect> for BoundingBox<f32> {
    fn from(rect: &RotatedRect) -> Self {
        let c = rect.a.cos().abs();
        let s = rect.a.sin().abs();

        let ex = rect.w / 2.0;
        let ey = rect.h / 2.0;

        let x_radius = ex * c + ey * s;
        let y_radius = ex * s + ey * c;

        BoundingBox::<f32>::new(
            rect.x - x_radius,
            rect.y - y_radius,
            rect.x + x_radius,
            rect.y + y_radius,
        )
    }
}

pub fn intersects<T>(item: &T, other: &T) -> bool
where
    T: Intersection,
{
    item.intersects(other)
}

// Rotated Rectangles Collision Detection, Oren Becker, 2001
impl Intersection for RotatedRect {
    fn intersects(&self, other: &Self) -> bool {
        let ang = self.a - other.a;
        let cosa = ang.cos();
        let sina = ang.sin();

        // Move rr2 to make rr1 cannonic
        let c = Vector(other.x, other.y) - Vector(self.x, self.y);

        // Rotate rr2 clockwise by rr2->ang to make rr2 axis-aligned
        let c = c.rotate(other.a);

        // Calculate vertices of (moved and axis-aligned := 'ma') rr2
        let size = Vector(other.w / 2.0, other.h / 2.0);
        let bl = c - size;
        let tr = c + size;

        // Calculate vertices of (rotated := 'r') rr1
        let (a, b) = {
            let cost = self.w / 2.0 * cosa;
            let sint = self.w / 2.0 * sina;

            let ax = -self.h / 2.0 * sina;
            let ay = self.h / 2.0 * cosa;

            (Vector(ax + cost, ay + sint), Vector(ax - cost, ay - sint))
        };

        // Verify that A is vertical min/max, B is horizontal min/max
        let t = sina * cosa;
        let (a, b) = {
            if t < 0.0 {
                (b, a)
            } else {
                (a, b)
            }
        };

        // Verify that B is horizontal minimum (left-most vertex)
        let b = if sina < 0.0 { -b } else { b };

        // If rr2(ma) isn't in the horizontal range of
        // colliding with rr1(r), collision is impossible
        if b.0 > tr.0 || b.0 > -(bl.0) {
            return false;
        }

        // This defaults correspond to the case of axis-alignment.
        let mut ext1 = a.1;
        let mut ext2 = -a.1;

        if t != 0.0 {
            {
                let mut dx1 = bl.0 - a.0;
                let dx2 = tr.0 - a.0;

                if dx1 * dx2 > 0.0 {
                    let mut dx = a.0;
                    if dx1 < 0.0 {
                        dx -= b.0;
                        ext1 -= b.1;
                        dx1 = dx2;
                    } else {
                        dx += b.0;
                        ext1 += b.1;
                    }

                    ext1 *= dx1;
                    ext1 /= dx;
                    ext1 += a.1;
                }
            }

            {
                let mut dx1 = bl.0 + a.0;
                let dx2 = tr.0 + a.0;

                if dx1 * dx2 > 0.0 {
                    let mut dx = -a.0;
                    if dx1 < 0.0 {
                        dx -= b.0;
                        ext2 -= b.1;
                        dx1 = dx2;
                    } else {
                        dx += b.0;
                        ext2 += b.1;
                    }
                    ext2 *= dx1;
                    ext2 /= dx;
                    ext2 -= a.1;
                }
            }
        }

        let (ext1, ext2) = if ext1 > ext2 {
            (ext2, ext1)
        } else {
            (ext1, ext2)
        };

        !((ext1 < bl.1 && ext2 < bl.1) || (ext1 > tr.1 && ext2 > tr.1))
    }
}
