// Problem description: https://leetcode.com/problems/number-of-islands/

#![allow(dead_code)]

struct Solution;

impl Solution {
    fn recursive_fill(grid: &mut [Vec<char>], i: i32, j: i32) {
        if !(0..grid.len() as i32).contains(&i) || !(0..grid[0].len() as i32).contains(&j) {
            return;
        }

        if grid[i as usize][j as usize] == '0' {
            return;
        }

        grid[i as usize][j as usize] = '0';
        Self::recursive_fill(grid, i - 1, j);
        Self::recursive_fill(grid, i + 1, j);
        Self::recursive_fill(grid, i, j - 1);
        Self::recursive_fill(grid, i, j + 1);
    }

    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut n = 0;

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == '1' {
                    Self::recursive_fill(&mut grid, i as i32, j as i32);
                    n += 1;
                }
            }
        }

        n
    }
}

#[test]
fn do_test() {
    let grid = vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ];

    assert_eq!(Solution::num_islands(grid), 1);

    let grid = vec![
        vec!['1', '1', '0', '0', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '1', '0', '0'],
        vec!['0', '0', '0', '1', '1'],
    ];

    assert_eq!(Solution::num_islands(grid), 3);
}
