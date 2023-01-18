// https://leetcode.com/problems/rotate-image/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        let mut do_rotate = |x: usize, y: usize| {
            let tmp = matrix[x][y];
            matrix[x][y] = matrix[n - y - 1][x];
            matrix[n - y - 1][x] = matrix[n - x - 1][n - y - 1];
            matrix[n - x - 1][n - y - 1] = matrix[y][n - x - 1];
            matrix[y][n - x - 1] = tmp;
        };

        for x in 0..(n + 1) / 2 {
            for y in 0..n / 2 {
                do_rotate(x, y);
            }
        }
    }
}

#[test]
fn do_test() {
    let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let expected = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];
    Solution::rotate(&mut matrix);
    assert_eq!(matrix, expected);

    let mut matrix = vec![
        vec![5, 1, 9, 11],
        vec![2, 4, 8, 10],
        vec![13, 3, 6, 7],
        vec![15, 14, 12, 16],
    ];
    let expected = vec![
        vec![15, 13, 2, 5],
        vec![14, 3, 4, 1],
        vec![12, 6, 8, 9],
        vec![16, 7, 10, 11],
    ];
    Solution::rotate(&mut matrix);
    assert_eq!(matrix, expected);
}
