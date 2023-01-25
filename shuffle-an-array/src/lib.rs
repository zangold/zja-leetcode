// Problem description: https://leetcode.com/problems/shuffle-an-array/

#![allow(dead_code)]

use rand::prelude::*;

struct Solution {
    nums: Vec<i32>,
}

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Self { nums }
    }

    fn reset(&self) -> Vec<i32> {
        self.nums.clone()
    }

    fn shuffle(&self) -> Vec<i32> {
        let mut v = self.nums.clone();

        for i in 0..v.len() - 1 {
            let swap_index = (rand::thread_rng().gen::<usize>() % (v.len() - i)) + i;

            v.swap(i, swap_index);
        }

        v
    }
}

#[test]
fn do_test() {
    let s = Solution::new(vec![1, 2, 3]);
    s.shuffle();
    assert_eq!(s.reset(), vec![1, 2, 3]);
    s.shuffle();
}
