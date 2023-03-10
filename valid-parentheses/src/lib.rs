// https://leetcode.com/problems/valid-parentheses/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();

        for c in s.chars() {
            match (c, stack.last().cloned()) {
                ('{', _) | ('[', _) | ('(', _) => {
                    stack.push(c);
                }
                ('}', Some('{')) => {
                    stack.pop();
                }
                (')', Some('(')) => {
                    stack.pop();
                }
                (']', Some('[')) => {
                    stack.pop();
                }
                (_, _) => {
                    return false;
                }
            }
        }

        stack.is_empty()
    }
}

#[test]
fn do_test() {
    assert!(Solution::is_valid("".to_string()));
    assert!(Solution::is_valid("()[]{}".to_string()));
    assert!(!Solution::is_valid("(]".to_string()));
    assert!(!Solution::is_valid("]".to_string()));
    assert!(!Solution::is_valid("[".to_string()));
    assert!(Solution::is_valid("({[]})".to_string()));
    assert!(!Solution::is_valid("({[][})".to_string()));
}
