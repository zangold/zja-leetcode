#![allow(dead_code)]

struct Solution;

impl Solution {
    fn intersects(a: &[i32], b: &[i32]) -> bool {
        !((a[1] < b[0]) || (a[0] > b[1]))
    }

    // Assumes that a and b overlap
    fn union(a: &[i32], b: &[i32]) -> Vec<i32> {
        vec![a[0].min(b[0]), a[1].max(b[1])]
    }

    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable_by_key(|i| i[0]);

        let mut result: Vec<Vec<i32>> = vec![];

        for interval in intervals {
            if let Some(last) = result.last_mut() {
                if Self::intersects(last, &interval) {
                    *last = Self::union(last, &interval);
                } else {
                    result.push(interval);
                }
            } else {
                result.push(interval);
            }
        }

        result
    }
}

#[test]
fn do_test() {
    let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
    let expected = vec![vec![1, 6], vec![8, 10], vec![15, 18]];
    assert_eq!(Solution::merge(intervals), expected);

    let intervals = vec![vec![1, 4], vec![4, 5]];
    let expected = vec![vec![1, 5]];
    assert_eq!(Solution::merge(intervals), expected);
}
