// https://leetcode.com/problems/pascals-triangle/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let num_rows = num_rows as usize;
        let mut tri = (1..=num_rows)
            .map(|r| vec![0i32; r])
            .collect::<Vec<Vec<i32>>>();

        tri[0][0] = 1;
        for row in 1..num_rows {
            tri[row][0] = 1;
            tri[row][row] = 1;

            for col in 1..row {
                tri[row][col] = tri[row - 1][col - 1] + tri[row - 1][col];
            }
        }

        tri
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::generate(1), vec![vec![1]]);

    assert_eq!(
        Solution::generate(5),
        vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1]
        ]
    );
}
