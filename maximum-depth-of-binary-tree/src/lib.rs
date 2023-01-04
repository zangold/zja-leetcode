#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<Self>>>,
    pub right: Option<Rc<RefCell<Self>>>,
}

impl TreeNode {
    #[inline]
    pub const fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                1 + i32::max(
                    Self::max_depth(node.borrow().left.clone()),
                    Self::max_depth(node.borrow().right.clone()),
                )
            }
        }
    }
}

fn make_node(
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

#[test]
fn do_test() {
    let tree = make_node(
        3,
        make_node(9, None, None),
        make_node(20, make_node(15, None, None), make_node(7, None, None)),
    );
    assert_eq!(Solution::max_depth(tree), 3);

    let tree = make_node(1, None, make_node(2, None, None));
    assert_eq!(Solution::max_depth(tree), 2);
}
