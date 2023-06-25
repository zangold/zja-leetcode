// Problem description: https://leetcode.com/problems/isomorphic-strings/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        // from s to t
        let mut char_map = ['\0'; 256];

        // from t to s
        let mut inv_char_map = ['\0'; 256];

        for (sc, tc) in s.chars().zip(t.chars()) {
            if char_map[sc as usize] != '\0' && char_map[sc as usize] != tc {
                return false;
            } else {
                char_map[sc as usize] = tc;
            }

            if inv_char_map[tc as usize] != '\0' && inv_char_map[tc as usize] != sc {
                return false;
            } else {
                inv_char_map[tc as usize] = sc;
            }
        }

        true
    }
}

#[test]
fn do_test() {
    assert!(Solution::is_isomorphic("egg".into(), "add".into()));
    assert!(!Solution::is_isomorphic("foo".into(), "bar".into()));
    assert!(Solution::is_isomorphic("paper".into(), "title".into()));
    assert!(Solution::is_isomorphic("x x".into(), "aha".into()));
}
