// https://leetcode.com/problems/word-search-ii/

#![allow(dead_code)]

const ALPHABET_LEN: usize = 26;

#[derive(Default)]
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

    pub fn access_next(&mut self, c: char) -> &mut Option<Box<Self>> {
        &mut self.children[char_index(c)]
    }

    pub fn try_prune(&mut self, c: char) {
        let ci = char_index(c);
        if let Some(ref mut child) = self.children[ci] {
            if child.is_empty() {
                self.children[ci] = None;
            }
        }
    }
}

struct Solution;

impl Solution {
    pub fn find_words_helper(
        board: &[Vec<char>],
        prefix: &mut Vec<char>,
        used: &mut Vec<Vec<bool>>,
        trie: &mut AlphaTrie,
        found_words: &mut Vec<String>,
        i: i32,
        j: i32,
    ) {
        if trie.is_word {
            found_words.push(prefix.iter().collect());
            trie.is_word = false;
        }

        let mut try_recurse = |di: i32, dj: i32| {
            // Bounds checking
            if !(0..board.len() as i32).contains(&(i + di))
                || !(0..board[0].len() as i32).contains(&(j + dj))
            {
                return;
            }

            let ni = (i + di) as usize;
            let nj = (j + dj) as usize;

            // Has this board cell already been used?
            if used[ni][nj] {
                return;
            }

            if let Some(ref mut next_trie) = trie.access_next(board[ni][nj]) {
                prefix.push(board[ni][nj]);
                used[ni][nj] = true;

                Self::find_words_helper(
                    board,
                    prefix,
                    used,
                    next_trie,
                    found_words,
                    ni as i32,
                    nj as i32,
                );

                used[ni][nj] = false;
                prefix.pop();
                trie.try_prune(board[ni][nj]);
            }
        };

        // check each of the directions
        try_recurse(-1, 0);
        try_recurse(1, 0);
        try_recurse(0, -1);
        try_recurse(0, 1);
    }

    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        // Process words into a trie
        let mut trie = AlphaTrie::new();
        words
            .iter()
            .for_each(|w| trie.add(&w.chars().collect::<Vec<char>>()));

        let mut found_words = Vec::<String>::new();

        // Persistent state for our recursive helper function
        let mut prefix = Vec::<char>::new();
        let mut used = vec![vec![false; board[0].len()]; board.len()];

        for i in 0..board.len() as i32 {
            for j in 0..board[0].len() as i32 {
                // determine which words can be found starting at board[i][j]
                if let Some(ref mut next_trie) = trie.access_next(board[i as usize][j as usize]) {
                    prefix.push(board[i as usize][j as usize]);
                    used[i as usize][j as usize] = true;

                    Self::find_words_helper(
                        &board,
                        &mut prefix,
                        &mut used,
                        next_trie,
                        &mut found_words,
                        i,
                        j,
                    );

                    used[i as usize][j as usize] = false;
                    prefix.pop();
                }
            }
        }

        found_words
    }
}

fn test_helper(board: Vec<Vec<char>>, words: Vec<&str>, expected: Vec<&str>) -> bool {
    let words: Vec<String> = words.iter().map(|w| w.to_string()).collect();
    let expected: Vec<String> = expected.iter().map(|w| w.to_string()).collect();

    let solved = Solution::find_words(board, words);

    if solved != expected {
        println!("actual: {:?}", solved);
        println!("expected: {:?}", expected);
        false
    } else {
        true
    }
}

#[test]
fn do_test() {
    let board = vec![
        vec!['o', 'a', 'a', 'n'],
        vec!['e', 't', 'a', 'e'],
        vec!['i', 'h', 'k', 'r'],
        vec!['i', 'f', 'l', 'v'],
    ];

    let words = vec!["oath", "pea", "eat", "rain"];
    let expected = vec!["oath", "eat"];

    assert!(test_helper(board, words, expected));

    let board = vec![vec!['a', 'b'], vec!['c', 'd']];

    let words = vec!["abcb"];
    let expected = Vec::<&str>::new();

    assert!(test_helper(board, words, expected));

    let board = vec![vec!['a']];

    let words = vec!["a"];
    let expected = vec!["a"];

    assert!(test_helper(board, words, expected));
}
