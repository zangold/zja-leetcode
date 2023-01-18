// https://leetcode.com/problems/valid-palindrome/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s: Vec<char> = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();

        let n = s.len() / 2;

        s[..n]
            .iter()
            .zip(s[s.len() - n..].iter().rev())
            .all(|(a, b)| a == b)
    }
}

#[test]
fn do_test() {
    assert!(Solution::is_palindrome(
        "A man, a plan, a canal: Panama".into()
    ));
    assert!(!Solution::is_palindrome("race a car".into()));
    assert!(Solution::is_palindrome(" (}[&(?)]) ".into()));
    assert!(Solution::is_palindrome("".into()));
    assert!(Solution::is_palindrome("race car".into()));
    assert!(!Solution::is_palindrome("race acr".into()));
    assert!(Solution::is_palindrome("bab".into()));
    assert!(!Solution::is_palindrome("bac".into()));
    assert!(!Solution::is_palindrome("cab".into()));
    assert!(Solution::is_palindrome("baab".into()));
    assert!(!Solution::is_palindrome("baxb".into()));
    assert!(!Solution::is_palindrome("bxab".into()));
}
