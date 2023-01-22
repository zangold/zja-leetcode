// Problem description: https://leetcode.com/problems/palindrome-partitioning/

#![allow(dead_code)]

struct Solution;

impl Solution {
    fn is_palindrome(s: &[char]) -> bool {
        for i in 0..s.len() / 2 {
            if s[i] != s[s.len() - i - 1] {
                return false;
            }
        }
        true
    }

    fn helper(s: &[char], result_prefix: &mut Vec<String>, results: &mut Vec<Vec<String>>) {
        if s.is_empty() {
            results.push(result_prefix.clone());
            return;
        }

        for i in 1..=s.len() {
            if Self::is_palindrome(&s[..i]) {
                result_prefix.push(s[..i].iter().collect());
                Self::helper(&s[i..], result_prefix, results);
                result_prefix.pop();
            }
        }
    }

    pub fn partition(s: String) -> Vec<Vec<String>> {
        let s = s.chars().collect::<Vec<_>>();
        let mut results = vec![];

        Self::helper(&s, &mut vec![], &mut results);
        results
    }
}

#[test]
fn do_test() {
    assert_eq!(
        Solution::partition("aab".into()),
        vec![
            vec!["a".to_string(), "a".to_string(), "b".to_string(),],
            vec!["aa".to_string(), "b".to_string(),],
        ]
    );

    assert_eq!(Solution::partition("a".into()), vec![vec!["a".to_string()]]);
}
