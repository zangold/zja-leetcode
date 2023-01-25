// Problem description: https://leetcode.com/problems/surrounded-regions/

#![allow(dead_code)]

struct Solution;

impl Solution {
    // Convert O to Z in this contiguous region
    fn helper(board: &mut [Vec<char>], i: i32, j: i32) {
        if !(0..board.len() as i32).contains(&i) || !(0..board[0].len() as i32).contains(&j) {
            return;
        }

        if board[i as usize][j as usize] != 'O' {
            return;
        }

        board[i as usize][j as usize] = 'Z';

        Self::helper(board, i - 1, j);
        Self::helper(board, i + 1, j);
        Self::helper(board, i, j - 1);
        Self::helper(board, i, j + 1);
    }

    pub fn solve(board: &mut Vec<Vec<char>>) {
        let m = board.len() as i32;
        let n = board[0].len() as i32;

        for i in 0..m {
            Self::helper(board, i, 0);
            Self::helper(board, i, n - 1);
        }

        for j in 1..n - 1 {
            Self::helper(board, 0, j);
            Self::helper(board, m - 1, j);
        }

        // Convert O to X, convert Z to O
        for row in board.iter_mut() {
            for cell in row.iter_mut() {
                *cell = match *cell {
                    'O' => 'X',
                    'Z' => 'O',
                    c => c,
                };
            }
        }
    }
}

#[test]
fn do_test() {
    let mut board = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'O', 'X'],
        vec!['X', 'X', 'O', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];

    let expected = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];

    Solution::solve(&mut board);
    assert_eq!(board, expected);

    let mut board = vec![vec!['X']];
    let expected = vec![vec!['X']];

    Solution::solve(&mut board);
    assert_eq!(board, expected);
}
