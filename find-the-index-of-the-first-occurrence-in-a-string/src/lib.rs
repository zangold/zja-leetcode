#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack = haystack.chars().collect::<Vec<char>>();
        let needle = needle.chars().collect::<Vec<char>>();

        if needle.len() > haystack.len() {
            return -1;
        }

        'a: for i in 0..=haystack.len() - needle.len() {
            for j in 0..needle.len() {
                if haystack[i + j] != needle[j] {
                    continue 'a;
                }
            }

            return i as i32;
        }

        -1
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::str_str("sadbutsad".into(), "sad".into()), 0);
    assert_eq!(Solution::str_str("sadbutsad".into(), "but".into()), 3);
    assert_eq!(Solution::str_str("leetcode".into(), "leeto".into()), -1);
}
