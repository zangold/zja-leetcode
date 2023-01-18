// https://leetcode.com/problems/binary-tree-maximum-path-sum/

#![allow(dead_code)]

use shared::*;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn helper(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<(i32, i32)> {
        if let Some(ref node) = root {
            let mut max_in_tree = i32::MIN;
            let mut max_in_tree_end_root = node.borrow().val;

            let mut passthrough_max = node.borrow().val;
            let mut passthrough_possible = true;

            if let Some((left_max, left_max_end_root)) = Self::helper(&node.borrow().left) {
                max_in_tree = max_in_tree.max(left_max);
                passthrough_max += left_max_end_root;
                max_in_tree_end_root =
                    max_in_tree_end_root.max(left_max_end_root + node.borrow().val);
            } else {
                passthrough_possible = false;
            }

            if let Some((right_max, right_max_end_root)) = Self::helper(&node.borrow().right) {
                max_in_tree = max_in_tree.max(right_max);
                passthrough_max += right_max_end_root;
                max_in_tree_end_root =
                    max_in_tree_end_root.max(right_max_end_root + node.borrow().val);
            } else {
                passthrough_possible = false;
            }

            if passthrough_possible {
                max_in_tree = max_in_tree.max(passthrough_max);
            }

            max_in_tree = max_in_tree.max(max_in_tree_end_root);
            Some((max_in_tree, max_in_tree_end_root))
        } else {
            None
        }
    }

    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::helper(&root).unwrap().0
    }
}

#[test]
fn do_test() {
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));

    assert_eq!(Solution::max_path_sum(tree), 6);

    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: -10,
        left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));

    assert_eq!(Solution::max_path_sum(tree), 42);

    let tree = Some(Rc::new(RefCell::new(TreeNode::new(-3))));
    assert_eq!(Solution::max_path_sum(tree), -3);

    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: None,
    })));
    assert_eq!(Solution::max_path_sum(tree), 3);
}
