// https://leetcode.com/problems/spiral-matrix/

#![allow(dead_code)]

struct Solution;

enum Dir {
    Right,
    Down,
    Left,
    Up,
}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        use Dir::*;
        let mut dir = Right;
        let mut rs = 0..matrix.len();
        let mut cs = 0..matrix[0].len();

        let mut order = Vec::<i32>::with_capacity(matrix.len() * matrix[0].len());

        while !rs.is_empty() && !cs.is_empty() {
            match dir {
                Right => {
                    let r = rs.start;
                    for c in cs.clone() {
                        order.push(matrix[r][c]);
                    }
                    rs.start += 1;
                    dir = Down;
                }
                Down => {
                    let c = cs.end - 1;
                    for r in rs.clone() {
                        order.push(matrix[r][c]);
                    }
                    cs.end -= 1;
                    dir = Left;
                }
                Left => {
                    let r = rs.end - 1;
                    for c in cs.clone().rev() {
                        order.push(matrix[r][c]);
                    }
                    rs.end -= 1;
                    dir = Up;
                }
                Up => {
                    let c = cs.start;
                    for r in rs.clone().rev() {
                        order.push(matrix[r][c]);
                    }
                    cs.start += 1;
                    dir = Right;
                }
            }
        }

        order
    }
}

#[test]
fn do_test() {
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let expected = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
    assert_eq!(Solution::spiral_order(matrix), expected);

    let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
    let expected = vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7];
    assert_eq!(Solution::spiral_order(matrix), expected);
}
