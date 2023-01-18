// https://leetcode.com/problems/sort-list/

#![allow(dead_code)]

use shared::*;

struct Solution;

impl Solution {
    pub fn list_from_slice(slice: &[i32]) -> Option<Box<ListNode>> {
        if slice.is_empty() {
            None
        } else {
            Some(Box::new(ListNode {
                val: slice[0],
                next: Self::list_from_slice(&slice[1..]),
            }))
        }
    }

    pub fn list_into_vec(head: Option<Box<ListNode>>, v: &mut Vec<i32>) {
        if let Some(node) = head {
            v.push(node.val);
            Self::list_into_vec(node.next, v);
        }
    }

    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut v = vec![];
        Self::list_into_vec(head, &mut v);
        v.sort_unstable();
        Self::list_from_slice(&v)
    }
}

#[test]
fn do_test() {
    assert_eq!(
        Solution::sort_list(ListNode::from_slice(&[4, 2, 1, 3])),
        ListNode::from_slice(&[1, 2, 3, 4])
    );
    assert_eq!(
        Solution::sort_list(ListNode::from_slice(&[])),
        ListNode::from_slice(&[])
    );
}
