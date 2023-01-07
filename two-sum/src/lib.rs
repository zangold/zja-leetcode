#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut indices = (0..nums.len()).collect::<Vec<_>>();
        indices.sort_by(|l, r| nums[*l].partial_cmp(&nums[*r]).unwrap());

        // searches for the second number
        let mut cursor = indices.len() - 1;

        for i in 0..indices.len() {
            while cursor > i && nums[indices[i]] + nums[indices[cursor]] > target {
                cursor -= 1
            }

            if cursor == i {
                panic!("Couldn't find a solution");
            }

            if nums[indices[i]] + nums[indices[cursor]] == target {
                return vec![indices[i] as i32, indices[cursor] as i32];
            }
        }

        panic!("Couldn't find a solution");
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
}
