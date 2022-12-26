#![allow(dead_code)]

struct Solution;

impl Solution {
    fn do_zeroing(matrix: &mut Vec<Vec<i32>>, r_prime: usize, c_prime: usize) {
        // this is probably pretty slow, but it's O(1) memory usage.
        for r in 0..matrix.len() {
            for c in 0..matrix[0].len() {
                if matrix[r][c] == 0 {
                    matrix[r][c_prime] = 0;
                    matrix[r_prime][c] = 0;
                }
            }
        }

        for r in 0..matrix.len() {
            for c in 0..matrix[0].len() {
                if r == r_prime || c == c_prime {
                    continue;
                }

                if matrix[r_prime][c] == 0 || matrix[r][c_prime] == 0 {
                    matrix[r][c] = 0;
                }
            }
        }

        for row in matrix.iter_mut() {
            row[c_prime] = 0;
        }

        for c in 0..matrix[0].len() {
            matrix[r_prime][c] = 0;
        }
    }

    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        // Seek to the first zero in the matrix. We'll end up zeroing out the row/col it's on, so
        // reuse that space.
        for r in 0..matrix.len() {
            for c in 0..matrix[0].len() {
                if matrix[r][c] == 0 {
                    Self::do_zeroing(matrix, r, c);
                    return;
                }
            }
        }
    }
}

#[test]
fn do_test() {
    let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    let expected = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];

    Solution::set_zeroes(&mut matrix);
    assert_eq!(matrix, expected);

    let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
    let expected = vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]];

    Solution::set_zeroes(&mut matrix);
    assert_eq!(matrix, expected);
}
