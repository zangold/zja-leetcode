#![allow(dead_code)]

struct Solution;

const ALPHABET_LEN: usize = 26;

#[derive(Debug, Default)]
struct AlphaTrie {
    pub is_word: bool,
    children: [Option<Box<AlphaTrie>>; ALPHABET_LEN],
}

fn char_index(c: char) -> usize {
    assert!(('a'..='z').contains(&c));
    c as usize - 'a' as usize
}

impl AlphaTrie {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn add(&mut self, s: &[char]) {
        if s.is_empty() {
            self.is_word = true;
            return;
        }

        let ci = char_index(s[0]);

        if let Some(ref mut child) = self.children[ci] {
            child.add(&s[1..]);
        } else {
            let mut new_child = Self::new();
            new_child.add(&s[1..]);
            self.children[ci] = Some(Box::new(new_child));
        }
    }

    pub fn remove(&mut self, s: &[char]) {
        if s.is_empty() {
            self.is_word = false;
            return;
        }

        let ci = char_index(s[0]);

        if let Some(ref mut child) = self.children[ci] {
            child.remove(&s[1..]);
            if child.is_empty() {
                self.children[ci] = None;
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        !self.is_word && self.children.iter().all(|c| c.is_none())
    }

    pub fn has_next(&self, c: char) -> bool {
        self.children[char_index(c)].is_some()
    }

    pub fn access_next(&self, c: char) -> &Option<Box<Self>> {
        &self.children[char_index(c)]
    }

    pub fn contains(&self, s: &[char]) -> bool {
        if s.is_empty() {
            self.is_word
        } else if let Some(ref child) = self.children[char_index(s[0])] {
            child.contains(&s[1..])
        } else {
            false
        }
    }
}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut word_trie = AlphaTrie::new();
        let s = s.chars().collect::<Vec<_>>();

        let mut dp = vec![false; s.len() + 1];
        dp[0] = true;

        for w in word_dict.into_iter() {
            word_trie.add(&w.chars().collect::<Vec<char>>());
        }

        for i in 1..=s.len() {
            for j in 0..i {
                if dp[j] && word_trie.contains(&s[j..i]) {
                    dp[i] = true;
                }
            }
        }

        dp[s.len()]
    }
}

#[test]
fn do_test() {
    assert!(Solution::word_break(
        "leetcode".into(),
        vec!["leet".to_string(), "code".to_string()]
    ));

    assert!(Solution::word_break(
        "applepenapple".into(),
        vec!["apple".to_string(), "pen".to_string()]
    ));

    assert!(!Solution::word_break(
        "catsandog".into(),
        vec![
            "cats".to_string(),
            "dog".to_string(),
            "sand".to_string(),
            "and".to_string(),
            "cat".to_string(),
        ]
    ));
}
