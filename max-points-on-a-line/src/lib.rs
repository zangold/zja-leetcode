#![allow(dead_code)]

struct Solution;

struct Line {
    base: (i32, i32),
    slope: (i32, i32),
}

impl Solution {
    /// Euclidean algorithm for GCD
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }

        a
    }

    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        // Change from vector-of-vectors into vector-of-tuples
        let points = points
            .into_iter()
            .map(|v| (v[0], v[1]))
            .collect::<Vec<(i32, i32)>>();

        // there's always going to be at least one point in the points vec.
        let mut max_points: i32 = 1;

        for (index, base) in points.iter().enumerate() {
            let mut points_copy = points[index + 1..].to_vec();

            while !points_copy.is_empty() {
                // Construct a line going from base to points_copy[0]. Count the number of points
                // in points_copy that it intersects, remove those points, and possibly save the
                // number of points as the current candidate return value.
                let p = points_copy.remove(0);

                // Find the slope (direction of the line) in lowest terms, so that we can determine
                // if a point 'x' is on our line when x - base is an integer multiple of slope.
                let slope = (p.0 - base.0, p.1 - base.1);
                let gcd = Self::gcd(slope.0.abs(), slope.1.abs());
                let slope = (slope.0 / gcd, slope.1 / gcd);

                let orig_len = points_copy.len();

                points_copy = points_copy
                    .into_iter()
                    .filter(|x| {
                        let y = (x.0 - base.0, x.1 - base.1);

                        let mut on_line = slope.0 == 0 || y.0 % slope.0 == 0;
                        on_line = on_line && (slope.1 == 0 || y.1 % slope.1 == 0);
                        on_line = on_line && (y.0 * slope.1 == y.1 * slope.0);

                        !on_line
                    })
                    .collect();

                let points_on_line = orig_len - points_copy.len() + 2;
                assert!(points_on_line >= 2);

                if max_points < points_on_line as i32 {
                    max_points = points_on_line as i32;
                }
            }
        }

        max_points
    }
}

#[test]
fn do_test() {
    assert_eq!(
        Solution::max_points(vec![vec![1, 1], vec![2, 2], vec![3, 3],]),
        3
    );

    assert_eq!(
        Solution::max_points(vec![
            vec![1, 1],
            vec![3, 2],
            vec![5, 3],
            vec![4, 1],
            vec![2, 3],
            vec![1, 4],
        ]),
        4
    );

    assert_eq!(
        Solution::max_points(vec![
            vec![1, 1],
            vec![2, 1],
            vec![3, 1],
            vec![4, 1],
            vec![2, 2],
            vec![3, 2],
        ]),
        4
    );

    assert_eq!(
        Solution::max_points(vec![
            vec![2, 1],
            vec![2, 2],
            vec![1, 1],
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
        ]),
        4
    );
}
