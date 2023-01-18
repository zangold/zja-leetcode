// https://leetcode.com/problems/remove-nth-node-from-end-of-list/

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
    fn helper(head: &mut Option<Box<ListNode>>, n: i32) -> i32 {
        let dist_from_end;
        if let Some(ref mut node) = head {
            dist_from_end = Self::helper(&mut node.next, n) + 1;
        } else {
            return 0;
        }

        if dist_from_end == n {
            if let Some(node) = head {
                *head = node.next.take();
            }
        }

        dist_from_end
    }

    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        Self::helper(&mut head, n);
        head
    }
}

#[test]
fn do_test() {
    let list = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 5, next: None })),
                })),
            })),
        })),
    }));

    let expected = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 5, next: None })),
            })),
        })),
    }));

    assert_eq!(Solution::remove_nth_from_end(list, 2), expected);

    assert_eq!(
        Solution::remove_nth_from_end(Some(Box::new(ListNode { val: 1, next: None })), 1),
        None
    );

    let list = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode { val: 2, next: None })),
    }));

    let expected = Some(Box::new(ListNode { val: 1, next: None }));

    assert_eq!(Solution::remove_nth_from_end(list, 1), expected);
}
