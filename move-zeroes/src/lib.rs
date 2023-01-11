#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut nz = 0;
        let mut i = 0;

        while i + nz < nums.len() {
            if nums[i + nz] == 0 {
                nz += 1;
            } else {
                nums[i] = nums[i + nz];
                i += 1;
            }
        }

        while i < nums.len() {
            nums[i] = 0;
            i += 1;
        }
    }
}

#[test]
fn do_test() {
    let mut nums = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut nums);
    assert_eq!(nums, vec![1, 3, 12, 0, 0]);

    let mut nums = vec![0];
    Solution::move_zeroes(&mut nums);
    assert_eq!(nums, vec![0]);
}
