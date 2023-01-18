// https://leetcode.com/problems/regular-expression-matching/

#![allow(dead_code)]

struct Solution;

impl Solution {
    fn is_match_helper(mut s: &[char], mut p: &[char]) -> bool {
        let matches = |sc, pc| {
            if pc == '.' {
                true
            } else {
                sc == pc
            }
        };

        while !p.is_empty() {
            if p.len() == 1 {
                if s.is_empty() || !matches(s[0], p[0]) {
                    return false;
                } else {
                    return s.len() == 1;
                }
            }

            match (p[0], p[1]) {
                (c, '*') => {
                    while !s.is_empty() && matches(s[0], c) {
                        // Try not matching this char, but continuing anyway.
                        if Self::is_match_helper(&s[0..], &p[2..]) {
                            return true;
                        }

                        s = &s[1..];
                    }
                    p = &p[1..];
                }
                (c, _) => {
                    if s.is_empty() {
                        return false;
                    } else if matches(s[0], c) {
                        s = &s[1..];
                    } else {
                        return false;
                    }
                }
            }

            p = &p[1..];
        }

        s.is_empty()
    }

    pub fn is_match(s: String, p: String) -> bool {
        Self::is_match_helper(
            &s.chars().collect::<Vec<char>>()[..],
            &p.chars().collect::<Vec<char>>()[..],
        )
    }
}

#[test]
fn do_test() {
    assert!(!Solution::is_match("aa".into(), "a".into()));
    assert!(Solution::is_match("aa".into(), "a*".into()));
    assert!(Solution::is_match("ab".into(), ".*".into()));

    // loop, looop, loooop, etc.
    assert!(Solution::is_match("looooop".into(), "loo*op".into()));

    assert!(Solution::is_match("aab".into(), "c*a*b".into()));
    assert!(Solution::is_match("a".into(), "ab*".into()));
}
