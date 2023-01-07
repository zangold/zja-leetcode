#![allow(dead_code)]

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
