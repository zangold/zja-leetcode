// https://leetcode.com/problems/merge-two-sorted-lists/

#![allow(dead_code)]

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<Self>>,
}

impl ListNode {
    #[inline]
    const fn new(val: i32) -> Self {
        Self { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut output: Option<Box<ListNode>> = None;
        let mut cursor: &mut Option<Box<ListNode>> = &mut output;

        while let (Some(ref node1), Some(ref node2)) = (&list1, &list2) {
            let val = i32::min(node1.val, node2.val);
            if node1.val < node2.val {
                list1 = list1.unwrap().next;
            } else {
                list2 = list2.unwrap().next;
            }

            *cursor = Some(Box::new(ListNode { val, next: None }));
            if let Some(node) = cursor {
                cursor = &mut node.next;
            }
        }

        if list1.is_some() {
            *cursor = list1;
        } else {
            *cursor = list2;
        }

        output
    }
}

fn make_list(mut values: &[i32]) -> Option<Box<ListNode>> {
    let mut list: Option<Box<ListNode>> = None;

    while let Some(value) = values.last() {
        list = Some(Box::new(ListNode {
            next: list,
            val: *value,
        }));
        values = &values[0..values.len() - 1];
    }

    list
}

#[test]
fn do_test() {
    let a = make_list(&[1, 2, 4]);
    let b = make_list(&[1, 3, 4]);
    let c = make_list(&[1, 1, 2, 3, 4, 4]);

    assert_eq!(Solution::merge_two_lists(a, b), c);

    let a = make_list(&[]);
    let b = make_list(&[]);
    let c = make_list(&[]);

    assert_eq!(Solution::merge_two_lists(a, b), c);

    let a = make_list(&[]);
    let b = make_list(&[0]);
    let c = make_list(&[0]);

    assert_eq!(Solution::merge_two_lists(a, b), c);
}
