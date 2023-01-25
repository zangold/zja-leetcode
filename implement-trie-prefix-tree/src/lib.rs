// Problem description: https://leetcode.com/problems/implement-trie-prefix-tree/

#![allow(dead_code)]

use std::str::Chars;

const ALPHABET_LEN: usize = 26;

#[derive(Default)]
struct Trie {
    pub is_word: bool,
    children: [Option<Box<Trie>>; ALPHABET_LEN],
}

fn char_index(c: char) -> usize {
    assert!(('a'..='z').contains(&c));
    c as usize - 'a' as usize
}

impl Trie {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn insert_chars(&mut self, mut chars: Chars<'_>) {
        if let Some(ch) = chars.next() {
            let ci = char_index(ch);

            if let Some(ref mut child) = self.children[ci] {
                child.insert_chars(chars);
            } else {
                let mut new_child = Self::new();
                new_child.insert_chars(chars);
                self.children[ci] = Some(Box::new(new_child));
            }
        } else {
            self.is_word = true;
        }
    }

    fn insert(&mut self, s: String) {
        self.insert_chars(s.chars())
    }

    fn search_chars(&self, mut chars: Chars<'_>) -> bool {
        if let Some(ch) = chars.next() {
            let ci = char_index(ch);

            if let Some(ref child) = self.children[ci] {
                child.search_chars(chars)
            } else {
                false
            }
        } else {
            self.is_word
        }
    }

    fn search(&self, word: String) -> bool {
        self.search_chars(word.chars())
    }

    fn starts_with_chars(&self, mut chars: Chars<'_>) -> bool {
        if let Some(ch) = chars.next() {
            let ci = char_index(ch);

            if let Some(ref child) = self.children[ci] {
                child.starts_with_chars(chars)
            } else {
                false
            }
        } else {
            // If this node exists, there must be a word in it or one of its children.
            true
        }
    }

    fn starts_with(&self, word: String) -> bool {
        self.starts_with_chars(word.chars())
    }
}

#[test]
fn do_test() {
    let mut trie = Trie::new();
    trie.insert("apple".into());
    assert!(trie.search("apple".into()));
    assert!(!trie.search("app".into()));
    assert!(trie.starts_with("app".into()));

    trie.insert("app".into());
    assert!(trie.search("app".into()));
}
