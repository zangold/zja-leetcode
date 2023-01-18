// https://leetcode.com/problems/max-sliding-window/

#![allow(dead_code)]

struct Solution;

use std::collections::BTreeMap;

impl Solution {
    pub fn add_to_window(window: &mut BTreeMap<i32, usize>, x: i32) {
        match window.get_mut(&x) {
            Some(count) => {
                *count += 1;
            }
            None => {
                window.insert(x, 1);
            }
        };
    }

    pub fn remove_from_window(window: &mut BTreeMap<i32, usize>, x: i32) {
        match window.get_mut(&x) {
            Some(count) => {
                if *count == 1 {
                    window.remove(&x);
                } else {
                    *count -= 1;
                }
            }
            None => {
                panic!("this shouldn't happen");
            }
        };
    }

    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut window = BTreeMap::<i32, usize>::new();
        let mut output = Vec::<i32>::new();

        let k = k as usize;

        nums[0..k]
            .iter()
            .for_each(|n| Self::add_to_window(&mut window, *n));
        output.push(*window.iter().next_back().unwrap().0);

        for i in 0..nums.len() - k {
            Self::remove_from_window(&mut window, nums[i]);
            Self::add_to_window(&mut window, nums[i + k]);
            output.push(*window.iter().next_back().unwrap().0);
        }

        output
    }
}

#[test]
fn do_test() {
    assert_eq!(
        Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
        vec![3, 3, 5, 5, 6, 7]
    );
    assert_eq!(Solution::max_sliding_window(vec![1], 1), vec![1]);

    assert_eq!(
        Solution::max_sliding_window(vec![1, 3, 3, -3, 5, 3, 6, 7], 2),
        vec![3, 3, 3, 5, 5, 6, 7]
    );
}
