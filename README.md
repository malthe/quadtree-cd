# [quadtree-cd](https://crates.io/crates/quadtree-cd)

A quadtree-based data structure for placing shapes such as rotated
rectangles in bounded 2D space, checking for collision with already
placed items.

[![crates.io
badge](https://img.shields.io/crates/v/quadtree-cd.svg)](https://crates.io/crates/quadtree-cd)
[![docs.rs
badge](https://docs.rs/quadtree-cd/badge.svg)](https://docs.rs/quadtree-cd)
[![license](https://img.shields.io/crates/l/quadtree-cd.svg)](https://github.com/malthe/quadtree-cd/blob/master/LICENSE)

For documentation, see [docs.rs/quadtree-cd](https://docs.rs/quadtree-cd/).

## Usage

The quadtree is initialized with a width and height parameter and is
generic on the type of geometry used; the package comes with support
for rotated rectangles.

The following example demonstrates how the quadtree can be used to
place rotated rectangles, checking for each placement whether it
intersects with already placed items.

```rust
use quadtree_cd::{Tree, BoundingBox, RotatedRect as Rect};

fn main() {
    let mut tree: Tree<RR> = Tree::new(1.0, 1.0);
    let rr1 = Rect { x: 0.5, y: 0.5, w: 0.5, h: 0.5, a: PI / 4.0 };
    let rr2 = Rect { x: 0.85, y: 0.85, w: 0.15, h: 0.15, a: PI / 8.0 };

    // These rectangles are non-intersecting.
    assert!(tree.insert_unless_intersecting(rr1, &(&rr1).into()));
    assert!(tree.insert_unless_intersecting(rr2, &(&rr2).into()));

    // But this one intersects at least one.
    let rr3 = Rect { x: 0.85, y: 0.85, w: 0.25, h: 0.25, a: PI / 8.0 };
    assert!(!tree.insert_unless_intersecting(rr3, &(&rr3).into()));
}
```
