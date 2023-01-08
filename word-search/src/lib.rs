#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn find_words_helper(board: &mut [Vec<char>], word: &[char], i: i32, j: i32) -> bool {
        if !(0..board.len() as i32).contains(&i) || !(0..board[0].len() as i32).contains(&j) {
            return false;
        }

        if board[i as usize][j as usize] != word[0] {
            return false;
        }

        if word.len() == 1 {
            return true;
        }

        board[i as usize][j as usize] = '?';

        let mut rec = |di, dj| Self::find_words_helper(board, &word[1..], i + di, j + dj);

        let found = rec(-1, 0) || rec(1, 0) || rec(0, -1) || rec(0, 1);

        board[i as usize][j as usize] = word[0];

        found
    }

    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let word = word.chars().collect::<Vec<_>>();

        for i in 0..board.len() as i32 {
            for j in 0..board[0].len() as i32 {
                if Self::find_words_helper(&mut board, &word, i, j) {
                    return true;
                }
            }
        }

        false
    }
}

#[test]
fn do_test() {
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];

    assert!(Solution::exist(board.clone(), "ABCCED".to_string()));
    assert!(Solution::exist(board.clone(), "SEE".to_string()));
    assert!(!Solution::exist(board, "ABCB".to_string()));
}
