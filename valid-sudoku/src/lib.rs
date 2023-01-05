#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let ch_index = |c| (c as u8 - b'1') as usize;

        let check_block = |xs: std::ops::Range<usize>, ys: std::ops::Range<usize>| {
            let mut seen = [false; 9];
            for x in xs {
                for y in ys.clone() {
                    if board[x][y] == '.' {
                        continue;
                    }

                    let ci = ch_index(board[x][y]);
                    if seen[ci] {
                        return false;
                    }
                    seen[ci] = true;
                }
            }
            true
        };

        // check 3x3 blocks
        for x in 0..3 {
            for y in 0..3 {
                if !check_block(x * 3..(x + 1) * 3, y * 3..(y + 1) * 3) {
                    return false;
                }
            }
        }

        // check rows
        for x in 0..9 {
            if !check_block(x..x + 1, 0..9) {
                return false;
            }
        }

        // check columns
        for y in 0..9 {
            if !check_block(0..9, y..y + 1) {
                return false;
            }
        }

        true
    }
}

#[test]
fn do_test() {
    let board = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    assert!(Solution::is_valid_sudoku(board));

    let board = vec![
        vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    assert!(!Solution::is_valid_sudoku(board));
}
