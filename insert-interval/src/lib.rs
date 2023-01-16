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

    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let index = match intervals.binary_search_by_key(&new_interval[0], |k| k[0]) {
            Ok(i) => i,
            Err(i) => i,
        };

        intervals.insert(index, new_interval);

        let mut new_intervals: Vec<Vec<i32>> = vec![];

        for interval in intervals {
            let next_interval;
            let mut overwrite = false;

            if let Some(last) = new_intervals.last() {
                if Self::intersects(&interval, last) {
                    next_interval = Self::union(&interval, last);
                    overwrite = true;
                } else {
                    next_interval = interval;
                }
            } else {
                next_interval = interval;
            }

            if overwrite {
                let index = new_intervals.len() - 1;
                new_intervals[index] = next_interval;
            } else {
                new_intervals.push(next_interval);
            }
        }

        new_intervals
    }
}

#[test]
fn do_test() {
    let intervals = vec![vec![1, 3], vec![6, 9]];
    let new_interval = vec![2, 5];
    let expected = vec![vec![1, 5], vec![6, 9]];

    assert_eq!(Solution::insert(intervals, new_interval), expected);

    let intervals = vec![
        vec![1, 2],
        vec![3, 5],
        vec![6, 7],
        vec![8, 10],
        vec![12, 16],
    ];
    let new_interval = vec![4, 8];
    let expected = vec![vec![1, 2], vec![3, 10], vec![12, 16]];

    assert_eq!(Solution::insert(intervals, new_interval), expected);
}
