#![allow(dead_code)]

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut hs = HashSet::new();
        hs.insert("".to_string());

        for _ in 0..n {
            let mut new_hs = HashSet::new();
            for s in hs {
                for i in 0..=s.len() {
                    let mut new_s = s.clone();
                    new_s.insert_str(i, "()");
                    new_hs.insert(new_s);
                }
            }

            hs = new_hs;
        }

        hs.into_iter().collect()
    }
}

#[test]
fn do_test() {
    let mut expected = vec![
        "()(())".to_string(),
        "()()()".to_string(),
        "((()))".to_string(),
        "(()())".to_string(),
        "(())()".to_string(),
    ];

    let mut actual = Solution::generate_parenthesis(3);
    expected.sort();
    actual.sort();

    assert_eq!(actual, expected);

    let expected = vec!["()".to_string()];

    assert_eq!(Solution::generate_parenthesis(1), expected);
}
