// https://leetcode.com/problems/first-unique-character-in-a-string/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut counts = [0; 26];
        let mut firsts = [0; 26];

        for (index, c) in s.chars().enumerate() {
            let ci = c as usize - b'a' as usize;
            counts[ci] += 1;

            if counts[ci] == 1 {
                firsts[ci] = index;
            }
        }

        let mut f: i32 = -1;

        for i in 0..26 {
            if counts[i] == 1 && (f == -1 || firsts[i] < firsts[f as usize]) {
                f = i as i32;
            }
        }

        if f == -1 {
            -1
        } else {
            firsts[f as usize] as i32
        }
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::first_uniq_char("leetcode".into()), 0);
    assert_eq!(Solution::first_uniq_char("loveleetcode".into()), 2);
    assert_eq!(Solution::first_uniq_char("aabb".into()), -1);
}
