#![allow(dead_code)]

use shared::*;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn helper(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            None
        } else {
            let pos = inorder.iter().position(|&n| n == preorder[0]).unwrap();
            Some(Rc::new(RefCell::new(TreeNode {
                val: preorder[0],
                left: Self::helper(&preorder[1..pos + 1], &inorder[..pos]),
                right: Self::helper(&preorder[pos + 1..], &inorder[pos + 1..]),
            })))
        }
    }

    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(&preorder, &inorder)
    }
}

#[test]
fn do_test() {
    let preorder = vec![3, 9, 20, 15, 7];
    let inorder = vec![9, 3, 15, 20, 7];
    let expected = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));

    assert_eq!(Solution::build_tree(preorder, inorder), expected);

    let preorder = vec![-1];
    let inorder = vec![-1];
    let expected = Some(Rc::new(RefCell::new(TreeNode::new(-1))));

    assert_eq!(Solution::build_tree(preorder, inorder), expected);
}
