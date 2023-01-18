// https://leetcode.com/problems/find-peak-element/

#![allow(dead_code)]

struct Solution;

use std::ops::Range;

// local shape of the function; i.e., the relative ordering of the three points
enum LocalShape {
    Peak,        // /\
    Valley,      // \/
    LeftRising,  // \
    RightRising, // /
}

use LocalShape::*;

impl Solution {
    pub fn check(nums: &[i32], index: i32) -> LocalShape {
        let a = if index == 0 {
            i64::MIN
        } else {
            nums[(index - 1) as usize] as i64
        };

        let b = nums[index as usize] as i64;

        let c = if index as usize == nums.len() - 1 {
            i64::MIN
        } else {
            nums[(index + 1) as usize] as i64
        };

        if b > a && b > c {
            Peak
        } else if a > b && b > c {
            LeftRising
        } else if a > b && c > b {
            Valley
        } else if a < b && b < c {
            RightRising
        } else {
            panic!("shouldn't be able to get here");
        }
    }

    pub fn helper(nums: &[i32], range: Range<i32>) -> i32 {
        let mid = (range.start + range.end) / 2;
        match Self::check(nums, mid) {
            Peak => mid,
            Valley => Self::helper(nums, range.start..mid),
            LeftRising => Self::helper(nums, range.start..mid),
            RightRising => Self::helper(nums, mid + 1..range.end),
        }
    }

    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        Self::helper(&nums, 0..nums.len() as i32)
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::find_peak_element(vec![1, 2, 3, 1]), 2);
    assert_eq!(Solution::find_peak_element(vec![1, 2, 1, 3, 5, 6, 4]), 5);

    assert_eq!(Solution::find_peak_element(vec![1, 2, 3, 4, 5, 6, 100]), 6);

    assert_eq!(Solution::find_peak_element(vec![100, 6, 5, 4, 3, 2, 1]), 0);
}
