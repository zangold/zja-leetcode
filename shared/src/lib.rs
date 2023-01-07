#![allow(dead_code)]

pub use std::cell::RefCell;
pub use std::rc::Rc;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<Self>>,
}

impl ListNode {
    #[inline]
    pub const fn new(val: i32) -> Self {
        Self { next: None, val }
    }

    pub fn from_slice(slice: &[i32]) -> Option<Box<Self>> {
        if slice.is_empty() {
            None
        } else {
            Some(Box::new(Self {
                val: slice[0],
                next: Self::from_slice(&slice[1..]),
            }))
        }
    }
}

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
