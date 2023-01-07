#![allow(dead_code)]

use shared::*;

struct Solution;

impl Solution {
    fn helper(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        let mid = nums.len() / 2;
        if nums.is_empty() {
            None
        } else {
            Some(Rc::new(RefCell::new(TreeNode {
                val: nums[mid],
                left: Self::helper(&nums[..mid]),
                right: Self::helper(&nums[mid + 1..]),
            })))
        }
    }

    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(&nums)
    }
}

#[test]
fn do_test() {
    let nums = vec![-10, -3, 0, 5, 9];
    let expected = Some(Rc::new(RefCell::new(TreeNode {
        val: 0,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: -3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(-10)))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            right: None,
        }))),
    })));

    assert_eq!(Solution::sorted_array_to_bst(nums), expected);

    let nums = vec![1, 3];
    let expected = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: None,
    })));

    assert_eq!(Solution::sorted_array_to_bst(nums), expected);
}
