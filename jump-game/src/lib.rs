#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        // inclusive; last index we can jump to.
        let mut max_jump: i32 = 0;

        for (i, jump) in nums.iter().enumerate() {
            if i as i32 > max_jump {
                return false;
            }

            max_jump = i32::max(max_jump, i as i32 + jump);

            if max_jump >= nums.len() as i32 - 1 {
                return true;
            }
        }

        // I don't think we'll ever get here, but return true if we do.
        true
    }
}

#[test]
fn do_test() {
    assert!(Solution::can_jump(vec![2, 3, 1, 1, 4]));
    assert!(!Solution::can_jump(vec![3, 2, 1, 0, 4]));
}
