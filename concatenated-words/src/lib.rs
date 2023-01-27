// Problem description: https://leetcode.com/problems/concatenated-words/

#![allow(dead_code)]

use std::str::Chars;

struct Solution;

struct AlphaTrie {
    children: [Option<Box<AlphaTrie>>; 26],
    is_word: bool,
}

fn char_index(c: char) -> usize {
    c as usize - 'a' as usize
}

impl AlphaTrie {
    fn new() -> Self {
        Self {
            children: Default::default(),
            is_word: false,
        }
    }

    fn insert(&mut self, mut chars: Chars<'_>) {
        if let Some(c) = chars.next() {
            let ci = char_index(c);
            if let Some(ref mut child) = self.children[ci] {
                child.insert(chars);
            } else {
                let mut new_child = Box::new(Self::new());
                new_child.insert(chars);
                self.children[ci] = Some(new_child);
            }
        } else {
            self.is_word = true;
        }
    }

    fn get_node(&self, mut chars: Chars<'_>) -> Option<&AlphaTrie> {
        if let Some(c) = chars.next() {
            let ci = char_index(c);

            if let Some(ref child) = self.children[ci] {
                child.get_node(chars)
            } else {
                None
            }
        } else {
            Some(self)
        }
    }

    // 0 -> not a word in the dictionary
    // 1 -> is a word in the dictionary
    // 2 -> is a concatenated word in the dictionary
    fn cat_word(&self, mut chars: Chars<'_>) -> usize {
        let mut current_node: &AlphaTrie = self;

        while let Some(c) = chars.next() {
            if let Some(ref child) = current_node.children[char_index(c)] {
                current_node = child;
            } else {
                return 0;
            }

            if current_node.is_word {
                // Recursive check
                if self.cat_word(chars.clone()) != 0 {
                    return 2;
                }
            }
        }

        if current_node.is_word {
            1
        } else {
            0
        }
    }
}

impl Solution {
    pub fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String> {
        let mut trie_root = AlphaTrie::new();

        for word in words.iter() {
            trie_root.insert(word.chars());
        }

        words
            .into_iter()
            .filter(|w| trie_root.cat_word(w.chars()) == 2)
            .collect()
    }
}

#[test]
fn do_test() {
    // test 1
    let words = vec![
        "cat".to_string(),
        "cats".to_string(),
        "catsdogcats".to_string(),
        "dog".to_string(),
        "dogcatsdog".to_string(),
        "hippopotamuses".to_string(),
        "rat".to_string(),
        "ratcatdogcat".to_string(),
    ];

    let expected = vec![
        "catsdogcats".to_string(),
        "dogcatsdog".to_string(),
        "ratcatdogcat".to_string(),
    ];

    assert_eq!(
        Solution::find_all_concatenated_words_in_a_dict(words),
        expected
    );

    // test 2
    let words = vec!["cat".to_string(), "dog".to_string(), "catdog".to_string()];
    let expected = vec!["catdog".to_string()];

    assert_eq!(
        Solution::find_all_concatenated_words_in_a_dict(words),
        expected
    );

    // test 3
    let words = vec!["a".to_string(), "aa".to_string()];
    let expected = vec!["aa".to_string()];

    assert_eq!(
        Solution::find_all_concatenated_words_in_a_dict(words),
        expected
    );
}
