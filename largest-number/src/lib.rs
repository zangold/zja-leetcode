// Problem description: https://leetcode.com/problems/largest-number/

#![allow(dead_code)]

use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut strs = nums
            .into_iter()
            .map(|n| format!("{n}"))
            .collect::<Vec<String>>();

        strs.sort_unstable_by(|a: &String, b: &String| -> Ordering {
            let ab = (a.clone() + b).parse::<usize>().unwrap();
            let ba = (b.clone() + a).parse::<usize>().unwrap();

            ab.cmp(&ba)
        });

        let r = strs
            .into_iter()
            .rev()
            .fold("".to_string(), |s, str| s + str.as_str());

        // wow, what an interesting corner case, leetcode.
        if let Some(c) = r.chars().next() {
            if c == '0' {
                return "0".to_string();
            }
        }

        r
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::largest_number(vec![10, 2]), "210".to_string());
    assert_eq!(
        Solution::largest_number(vec![3, 30, 34, 5, 9]),
        "9534330".to_string()
    );

    assert_eq!(Solution::largest_number(vec![0, 0]), "0".to_string());
}
