// https://leetcode.com/problems/maximum-bags-with-full-capacity-of-rocks/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn maximum_bags(mut capacity: Vec<i32>, rocks: Vec<i32>, mut additional_rocks: i32) -> i32 {
        capacity
            .iter_mut()
            .zip(rocks.iter())
            .for_each(|(cap, rocks)| *cap -= rocks);
        capacity.sort_unstable();

        let mut filled = 0;
        for c in capacity {
            if additional_rocks < c {
                break;
            }

            additional_rocks -= c;
            filled += 1;
        }

        filled
    }
}

#[test]
fn do_test() {
    let capacity = vec![2, 3, 4, 5];
    let rocks = vec![1, 2, 4, 4];
    let additional_rocks = 2;
    let expected = 3;

    assert_eq!(
        Solution::maximum_bags(capacity, rocks, additional_rocks),
        expected
    );

    let capacity = vec![10, 2, 2];
    let rocks = vec![2, 2, 0];
    let additional_rocks = 100;
    let expected = 3;

    assert_eq!(
        Solution::maximum_bags(capacity, rocks, additional_rocks),
        expected
    );
}
