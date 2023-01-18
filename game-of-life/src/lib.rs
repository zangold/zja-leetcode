// https://leetcode.com/problems/game-of-life/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let rows = 0..board.len() as i32;
        let cols = 0..board[0].len() as i32;

        let mut new_board = vec![vec![0i32; board[0].len()]; board.len()];

        let offsets: [(i32, i32); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for r in rows.clone() {
            for c in cols.clone() {
                let mut num_neighbours = 0;
                for offset in offsets {
                    let nr = r + offset.0;
                    let nc = c + offset.1;
                    if rows.contains(&nr)
                        && cols.contains(&nc)
                        && board[nr as usize][nc as usize] == 1
                    {
                        num_neighbours += 1;
                    }
                }

                new_board[r as usize][c as usize] = if board[r as usize][c as usize] == 1 {
                    (2..4).contains(&num_neighbours) as i32
                } else {
                    (num_neighbours == 3) as i32
                };
            }
        }

        *board = new_board
    }
}

#[test]
fn do_test() {
    let mut board = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
    let expected = vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]];
    Solution::game_of_life(&mut board);
    assert_eq!(board, expected);

    let mut board = vec![vec![1, 1], vec![1, 0]];
    let expected = vec![vec![1, 1], vec![1, 1]];
    Solution::game_of_life(&mut board);
    assert_eq!(board, expected);
}
