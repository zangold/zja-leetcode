// Problem description: https://leetcode.com/problems/binary-tree-level-order-traversal/

#![allow(dead_code)]

struct Solution;

use shared::*;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }

        let mut level = vec![root.unwrap()];
        let mut levels = vec![];

        while !level.is_empty() {
            let mut next_level = vec![];
            let mut level_nums = vec![];

            for node in level {
                level_nums.push(node.borrow().val);

                if let Some(ref left) = node.borrow().left {
                    next_level.push(left.clone());
                }

                if let Some(ref right) = node.borrow().right {
                    next_level.push(right.clone());
                }
            }

            levels.push(level_nums);
            level = next_level;
        }

        levels
    }
}

#[test]
fn do_test() {
    let root = tree_deserialize("[3,9,20,null,null,15,7]");
    let expected = vec![vec![3], vec![9, 20], vec![15, 7]];

    assert_eq!(Solution::level_order(root), expected);

    let root = tree_deserialize("[1]");
    let expected = vec![vec![1]];

    assert_eq!(Solution::level_order(root), expected);

    let root = tree_deserialize("[]");
    let expected: Vec<Vec<i32>> = vec![];

    assert_eq!(Solution::level_order(root), expected);
}
