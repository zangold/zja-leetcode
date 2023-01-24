// Problem description: https://leetcode.com/problems/snakes-and-ladders/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        // Flatten the board into one dimension
        let mut new_board = vec![];

        for i in (0..board.len()).rev() {
            if (board.len() - i) % 2 == 0 {
                for j in board[i].iter().rev() {
                    new_board.push(*j);
                }
            } else {
                for j in board[i].iter() {
                    new_board.push(*j);
                }
            }
        }

        assert_eq!(new_board.len(), board.len() * board.len());

        let mut dp = vec![-1; board.len() * board.len()];

        // 0 moves to get to the starting square, from the starting square
        dp[0] = 0;

        let mut i = 0;

        while i < dp.len() {
            let mut next_i = i + 1;

            if dp[i] == -1 {
                i = next_i;
                continue;
            }

            for roll in 1..=6 {
                if i + roll < dp.len() {
                    let mut square = (i + roll) as usize;

                    // follow a snake or ladder, if present
                    if new_board[square] != -1 {
                        square = new_board[square] as usize - 1;
                    }

                    if dp[square] == -1 || dp[square] > dp[i] + 1 {
                        dp[square] = dp[i] + 1;
                        next_i = next_i.min(square);
                    }
                }
            }

            i = next_i;
        }

        *dp.last().unwrap()
    }
}

#[test]
fn do_test() {
    let board = vec![
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, 35, -1, -1, 13, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, 15, -1, -1, -1, -1],
    ];

    assert_eq!(Solution::snakes_and_ladders(board), 4);

    let board = vec![vec![-1, -1], vec![-1, 3]];
    assert_eq!(Solution::snakes_and_ladders(board), 1);

    let board = vec![vec![1, 1, -1], vec![1, 1, 1], vec![-1, 1, 1]];
    assert_eq!(Solution::snakes_and_ladders(board), -1);

    let board = vec![vec![1, 1, -1], vec![1, -1, 1], vec![-1, 1, 1]];
    assert_eq!(Solution::snakes_and_ladders(board), 2);

    let board = vec![vec![1, 1, -1], vec![1, 1, 1], vec![-1, 1, -1]];
    assert_eq!(Solution::snakes_and_ladders(board), 2);

    let board = vec![vec![1, -1, -1], vec![1, 1, 1], vec![-1, -1, 1]];
    assert_eq!(Solution::snakes_and_ladders(board), 3);

    let board = vec![
        vec![-1, -1, -1, -1, 48, 5, -1],
        vec![12, 29, 13, 9, -1, 2, 32],
        vec![-1, -1, 21, 7, -1, 12, 49],
        vec![42, 37, 21, 40, -1, 22, 12],
        vec![42, -1, 2, -1, -1, -1, 6],
        vec![39, -1, 35, -1, -1, 39, -1],
        vec![-1, 36, -1, -1, -1, -1, 5],
    ];

    assert_eq!(Solution::snakes_and_ladders(board), 3);
}
