mod quadtree_cd_tests {
    use quadtree_cd::{BoundingBox, Intersection, RotatedRect as RR, Tree};
    use rand::rngs::SmallRng;
    use rand::{Rng, SeedableRng};
    use std::f32::consts::PI;
    use std::mem::transmute;

    const FUZZ_COUNT: usize = 100000;

    const RR1: RR = RR {
        x: 0.5,
        y: 0.5,
        w: 0.5,
        h: 0.5,
        a: PI / 4.0,
    };

    const RR2: RR = RR {
        x: 0.85,
        y: 0.85,
        w: 0.15,
        h: 0.15,
        a: PI / 8.0,
    };

    const RR3: RR = RR {
        x: 0.85,
        y: 0.85,
        w: 0.25,
        h: 0.25,
        a: PI / 8.0,
    };

    #[test]
    fn test_insert() {
        let mut tree: Tree<RR> = Tree::new(1.0, 1.0);
        tree.insert(RR1, &(&RR1).into());
        tree.insert(RR1, &(&RR2).into());
        tree.insert(RR1, &(&RR3).into());
    }

    #[test]
    fn test_insert_unless_intersecting() {
        let mut tree: Tree<RR> = Tree::new(1.0, 1.0);
        assert!(tree.insert_unless_intersecting(RR1, &(&RR1).into()));
        assert!(tree.insert_unless_intersecting(RR2, &(&RR2).into()));
        assert!(!tree.insert_unless_intersecting(RR3, &(&RR3).into()));
    }

    #[test]
    fn test_insert_twice() {
        let mut tree: Tree<RR> = Tree::new(1.0, 1.0);
        let rr = RR {
            x: 0.5,
            y: 0.5,
            w: 0.5,
            h: 0.5,
            a: PI / 4.0,
        };

        let bbox: BoundingBox<f32> = (&rr).into();
        assert_eq!(tree.insert_unless_intersecting(RR1.clone(), &bbox), true);
        assert_eq!(tree.insert_unless_intersecting(RR1.clone(), &bbox), false);
    }

    #[test]
    fn test_insert_grid() {
        let size = 128.0;

        for depth in 1..7 {
            let mut tree: Tree<RR> = Tree::new(size, size);
            let count = 2u16.pow(depth - 1);
            let u: f32 = size / count as f32;

            for i in 0..count {
                for j in 0..count {
                    let r = RR {
                        x: i as f32 * u + u / 2.0,
                        y: j as f32 * u + u / 2.0,
                        w: u / 2.0f32.sqrt() * 0.99,
                        h: u / 2.0f32.sqrt() * 0.99,
                        a: PI / 4.0,
                    };
                    let bbox: BoundingBox<f32> = (&r).into();
                    let inserted =
                        tree.insert_unless_intersecting(r.clone(), &bbox);
                    assert!(inserted);
                }
            }
        }
    }

    #[test]
    fn test_fuzz_intersect() {
        for i in 0..FUZZ_COUNT {
            // Random generator seeded with iteration.
            let mut rng = {
                let mut seed = [0; 16];
                let bytes: [u8; 8] = unsafe { transmute(i.to_be()) };
                seed.split_at_mut(8).0.copy_from_slice(&bytes);
                SmallRng::from_seed(seed)
            };

            let mut tree: Tree<RR> = Tree::new(1.0, 1.0);
            let mut rs: Vec<RR> = Vec::new();

            loop {
                let x = rng.gen();
                let y = rng.gen();
                let w = rng.gen();
                let h = rng.gen();

                let r = RR {
                    x: x,
                    y: y,
                    w: w,
                    h: h,
                    a: 2.0 * PI * rng.gen::<f32>(),
                };

                let b: BoundingBox<f32> = (&r).into();

                if b.x0 < 0.0 || b.x1 > 1.0 || b.y0 < 0.0 || b.y1 > 1.0 {
                    continue;
                }

                let intersects = rs.iter().any(|&other| r.intersects(&other));
                let inserted = tree.insert_unless_intersecting(r.clone(), &b);

                assert_eq!(inserted, !intersects);

                if intersects {
                    break;
                } else {
                    rs.push(r);
                }
            }
        }
    }
}
