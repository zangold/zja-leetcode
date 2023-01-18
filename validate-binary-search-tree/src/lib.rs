#![allow(dead_code)]

use shared::*;

struct Solution;

impl Solution {
    pub fn helper(
        node: &Option<Rc<RefCell<TreeNode>>>,
        values: std::ops::RangeInclusive<i64>,
    ) -> bool {
        match node {
            &None => true,
            Some(refcell) => {
                let node = refcell.borrow();

                if (node.val as i64) < *values.start() || (node.val as i64) > *values.end() {
                    false
                } else {
                    Self::helper(&node.left, *values.start()..=node.val as i64 - 1)
                        && Self::helper(&node.right, node.val as i64 + 1..=*values.end())
                }
            }
        }
    }

    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::helper(&root, i32::MIN as i64..=i32::MAX as i64)
    }
}

#[test]
fn do_test() {
    assert!(Solution::is_valid_bst(tree_deserialize("[2,1,3]")));
    assert!(!Solution::is_valid_bst(tree_deserialize(
        "[5,1,4,null,null,3,6]"
    )));
    assert!(!Solution::is_valid_bst(tree_deserialize(
        "[-2147483648,-2147483648]"
    )));
}
