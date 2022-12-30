#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<Self>>>,
    pub right: Option<Rc<RefCell<Self>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

impl Solution {
    fn helper(inorder: &mut Vec<i32>, node: &Option<Rc<RefCell<TreeNode>>>) {
        if node.is_none() {
            return;
        }

        if let Some(ref rt) = node {
            let t = rt.borrow();
            Self::helper(inorder, &t.left);
            inorder.push(t.val);
            Self::helper(inorder, &t.right);
        }
    }

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut inorder = Vec::<i32>::new();

        Self::helper(&mut inorder, &root);

        inorder
    }
}

#[test]
fn do_test() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
            right: None,
        }))),
    })));

    assert_eq!(Solution::inorder_traversal(root), vec![1, 3, 2]);

    let root = None;

    assert_eq!(Solution::inorder_traversal(root), vec![]);

    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: None,
    })));

    assert_eq!(Solution::inorder_traversal(root), vec![1]);

    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            }))),
        }))),
    })));

    assert_eq!(Solution::inorder_traversal(root), vec![1, 2, 3, 4, 5, 6, 7]);
}
