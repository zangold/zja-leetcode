// https://leetcode.com/problems/unique-paths-iii/

#![allow(dead_code)]

struct Solution;

const BLOCKED: i32 = -1;
const EMPTY: i32 = 0;
const START: i32 = 1;
const END: i32 = 2;

impl Solution {
    fn helper(grid: &mut [Vec<i32>], i: i32, j: i32, mut num_left: i32) -> i32 {
        if grid[i as usize][j as usize] == BLOCKED {
            return 0;
        } else if grid[i as usize][j as usize] == END {
            if num_left == 1 {
                return 1;
            } else {
                return 0;
            }
        }

        // Should have run into a dead end by now.
        assert!(num_left > 0);
        num_left -= 1;
        grid[i as usize][j as usize] = BLOCKED;

        let mut try_recurse = |di, dj| -> i32 {
            let ni = i + di;
            let nj = j + dj;

            if !(0..grid.len() as i32).contains(&ni) || !(0..grid[0].len() as i32).contains(&nj) {
                return 0;
            }

            Self::helper(grid, ni, nj, num_left)
        };

        let mut num_paths = 0;
        num_paths += try_recurse(-1, 0);
        num_paths += try_recurse(1, 0);
        num_paths += try_recurse(0, -1);
        num_paths += try_recurse(0, 1);

        grid[i as usize][j as usize] = EMPTY;

        num_paths
    }

    pub fn unique_paths_iii(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut start_i = grid.len();
        let mut start_j = grid[0].len();

        // Tracks the number of squares that need to be visited for the path to be complete.
        let mut num_left = 0;

        for (i, row) in grid.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell == START {
                    start_i = i;
                    start_j = j;
                    num_left += 1;
                } else if cell == EMPTY || cell == END {
                    num_left += 1;
                }
            }
        }

        assert!(start_i < grid.len());
        assert!(start_j < grid[0].len());

        Self::helper(&mut grid, start_i as i32, start_j as i32, num_left)
    }
}

#[test]
fn do_test() {
    let grid = vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 2, -1]];
    assert_eq!(Solution::unique_paths_iii(grid), 2);

    let grid = vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 2]];
    assert_eq!(Solution::unique_paths_iii(grid), 4);

    let grid = vec![vec![0, 1], vec![2, 0]];
    assert_eq!(Solution::unique_paths_iii(grid), 0);

    let grid = vec![vec![1, 0, 0, 0], vec![0, 0, -1, -1], vec![0, 0, -1, 2]];
    assert_eq!(Solution::unique_paths_iii(grid), 0);

    let grid = vec![vec![1, 0, 0, 2]];
    assert_eq!(Solution::unique_paths_iii(grid), 1);
}
