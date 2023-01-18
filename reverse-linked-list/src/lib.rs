// https://leetcode.com/problems/reverse-linked-list/

#![allow(dead_code)]

use shared::*;

struct Solution;

impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_list = None;

        while let Some(mut first) = head {
            let rest = first.next;
            first.next = new_list;
            new_list = Some(first);

            head = rest;
        }

        new_list
    }
}

#[test]
fn do_test() {
    let list = ListNode::from_slice(&[1, 2, 3, 4, 5]);
    let expected = ListNode::from_slice(&[5, 4, 3, 2, 1]);
    assert_eq!(Solution::reverse_list(list), expected);

    let list = ListNode::from_slice(&[1, 2]);
    let expected = ListNode::from_slice(&[2, 1]);
    assert_eq!(Solution::reverse_list(list), expected);
}
