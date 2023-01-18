// https://leetcode.com/problems/container-with-most-water/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;

        let mut left = 0;
        let mut right = height.len() - 1;

        while left != right {
            let area = i32::min(height[left], height[right]) * (right - left) as i32;
            max_area = i32::max(max_area, area);

            if height[left] > height[right] {
                right -= 1;
            } else {
                left += 1;
            }
        }

        max_area
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    assert_eq!(Solution::max_area(vec![1, 1]), 1);
    assert_eq!(Solution::max_area(vec![0, 1]), 0);
}
