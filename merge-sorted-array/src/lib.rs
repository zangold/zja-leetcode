// https://leetcode.com/problems/merge-sorted-array/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, _n: i32) {
        let nums1_clone = nums1[0..m as usize].to_vec();

        // some renaming
        let output = nums1;
        let mut nums1 = nums1_clone;

        let mut cursor = 0;

        loop {
            if nums1.is_empty() {
                while !nums2.is_empty() {
                    output[cursor] = nums2.remove(0);
                    cursor += 1;
                }
                break;
            } else if nums2.is_empty() {
                while !nums1.is_empty() {
                    output[cursor] = nums1.remove(0);
                    cursor += 1;
                }
                break;
            } else if nums1[0] < nums2[0] {
                output[cursor] = nums1.remove(0);
            } else {
                output[cursor] = nums2.remove(0);
            }

            cursor += 1;
        }
    }
}

#[test]
fn do_test() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];
    Solution::merge(&mut nums1, 3, &mut nums2, 3);
    assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);

    let mut nums1 = vec![1];
    let mut nums2 = vec![];
    Solution::merge(&mut nums1, 1, &mut nums2, 0);
    assert_eq!(nums1, vec![1]);

    let mut nums1 = vec![0];
    let mut nums2 = vec![1];
    Solution::merge(&mut nums1, 0, &mut nums2, 1);
    assert_eq!(nums1, vec![1]);

    let mut nums1 = vec![0, 2, 4, 6, 8, 0, 0, 0, 0, 0];
    let mut nums2 = vec![1, 3, 5, 7, 9];
    Solution::merge(&mut nums1, 5, &mut nums2, 5);
    assert_eq!(nums1, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

    let mut nums1 = vec![0, 2, 8, 0, 0, 0, 0, 0, 0, 0];
    let mut nums2 = vec![1, 3, 4, 5, 6, 7, 9];
    Solution::merge(&mut nums1, 3, &mut nums2, 7);
    assert_eq!(nums1, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

    let mut nums1 = vec![0, 2, 3, 4, 5, 6, 8, 0, 0, 0];
    let mut nums2 = vec![1, 7, 9];
    Solution::merge(&mut nums1, 7, &mut nums2, 3);
    assert_eq!(nums1, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}
