#![allow(dead_code)]

struct Solution;

impl Solution {
    fn helper(matrix: &[Vec<i32>], dp: &mut Vec<Vec<i32>>, a: usize, b: usize) {
        if dp[a][b] != 0 {
            return;
        }

        let v = matrix[a][b];

        // length of the longest increasing path starting from matrix[a][b]
        let mut best = 1i32;

        if a > 0 && matrix[a - 1][b] > v {
            Solution::helper(matrix, dp, a - 1, b);
            best = i32::max(best, dp[a - 1][b] + 1);
        }

        if a < matrix.len() - 1 && matrix[a + 1][b] > v {
            Solution::helper(matrix, dp, a + 1, b);
            best = i32::max(best, dp[a + 1][b] + 1);
        }

        if b > 0 && matrix[a][b - 1] > v {
            Solution::helper(matrix, dp, a, b - 1);
            best = i32::max(best, dp[a][b - 1] + 1);
        }

        if b < matrix[0].len() - 1 && matrix[a][b + 1] > v {
            Solution::helper(matrix, dp, a, b + 1);
            best = i32::max(best, dp[a][b + 1] + 1);
        }

        dp[a][b] = best;
    }

    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        // dp[a][b] stores length of the longest increasing path starting at matrix[a][b], or 0 if
        // it hasn't been computed yet.
        let mut dp = vec![vec![0i32; matrix[0].len()]; matrix.len()];

        let mut longest = 1i32;

        for a in 0..matrix.len() {
            for b in 0..matrix[0].len() {
                Solution::helper(&matrix, &mut dp, a, b);
                longest = i32::max(dp[a][b], longest);
            }
        }

        longest
    }
}

#[test]
fn do_test() {
    let matrix = vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]];
    assert_eq!(Solution::longest_increasing_path(matrix), 4);

    let matrix = vec![vec![3, 4, 5], vec![3, 2, 6], vec![2, 2, 1]];
    assert_eq!(Solution::longest_increasing_path(matrix), 4);

    let matrix = vec![vec![1]];
    assert_eq!(Solution::longest_increasing_path(matrix), 1);
}
