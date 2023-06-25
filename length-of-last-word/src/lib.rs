// Problem description: https://leetcode.com/problems/length-of-last-word/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim_end().split(' ').last().unwrap().len() as i32
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::length_of_last_word("Hello World".into()), 5);
    assert_eq!(
        Solution::length_of_last_word("  fly me   to the moon   ".into()),
        4
    );
    assert_eq!(
        Solution::length_of_last_word("luffy is still joyboy".into()),
        6
    );
}
