#![allow(dead_code)]

struct Solution;

use std::cmp::Ordering::*;

impl Solution {
    pub fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        nums1.sort_unstable();
        nums2.sort_unstable();

        let mut intersection = Vec::<i32>::with_capacity(usize::max(nums1.len(), nums2.len()));

        let mut i = 0;
        let mut j = 0;

        while i < nums1.len() && j < nums2.len() {
            match nums1[i].cmp(&nums2[j]) {
                Equal => {
                    intersection.push(nums1[i]);
                    i += 1;
                    j += 1;
                }
                Less => {
                    i += 1;
                }
                Greater => {
                    j += 1;
                }
            }
        }

        intersection
    }
}

#[test]
fn do_test() {
    assert_eq!(
        Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]),
        vec![2, 2]
    );
    assert_eq!(
        Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]),
        vec![4, 9]
    );
}
