#![allow(dead_code)]

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut pattern_map: [String; 26] = Default::default();
        let mut found_words = HashSet::<String>::new();
        let words = s.split(' ').collect::<Vec<&str>>();

        let ch_index = |c: char| (c as u8 - b'a') as usize;

        if words.len() != pattern.len() {
            return false;
        }

        for (p, word) in pattern.chars().zip(words.iter()) {
            let p_index = ch_index(p);
            let ws = word.to_string();

            if pattern_map[p_index].is_empty() {
                if found_words.contains(&ws) {
                    return false;
                } else {
                    found_words.insert(ws.clone());
                    pattern_map[p_index] = ws;
                }
            } else if pattern_map[p_index] != ws {
                return false;
            }
        }

        true
    }
}

#[test]
fn do_test() {
    assert!(Solution::word_pattern(
        "abba".into(),
        "dog cat cat dog".into()
    ));
    assert!(!Solution::word_pattern(
        "abba".into(),
        "dog cat cat fish".into()
    ));
    assert!(!Solution::word_pattern(
        "aaaa".into(),
        "dog cat cat dog".into()
    ));
    assert!(!Solution::word_pattern(
        "abba".into(),
        "dog dog dog dog".into()
    ));
}
