#![allow(dead_code)]

use shared::*;

struct Solution;

impl Solution {
    fn list_len(head: &Option<Box<ListNode>>) -> usize {
        if let Some(ref node) = head {
            1 + Self::list_len(&node.next)
        } else {
            0
        }
    }

    // Recurse halfway into the list, then as we unwind the stack, check the node on the opposite
    // end for equality.
    fn helper(head: &Option<Box<ListNode>>, n: usize, r: usize) -> (bool, &Option<Box<ListNode>>) {
        if let Some(ref node) = head {
            if r < (n - 1) / 2 {
                let (mut is_palindrome, next_after) = Self::helper(&node.next, n, r + 1);

                if let Some(next_after_node) = next_after {
                    is_palindrome = is_palindrome && node.val == next_after_node.val;
                    (is_palindrome, &next_after_node.next)
                } else {
                    (false, &None)
                }
            } else if n % 2 == 0 {
                if let Some(ref next_node) = node.next {
                    (node.val == next_node.val, &next_node.next)
                } else {
                    (false, &None)
                }
            } else {
                (true, &node.next)
            }
        } else {
            (false, &None)
        }
    }

    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let n = Self::list_len(&head);
        Self::helper(&head, n, 0).0
    }
}

#[test]
fn do_test() {
    assert!(Solution::is_palindrome(ListNode::from_slice(&[1, 2, 2, 1])));
    assert!(Solution::is_palindrome(ListNode::from_slice(&[
        1, 2, 3, 2, 1
    ])));
    assert!(Solution::is_palindrome(ListNode::from_slice(&[
        1, 2, 3, 3, 2, 1
    ])));
    assert!(Solution::is_palindrome(ListNode::from_slice(&[
        1, 2, 3, 4, 3, 2, 1
    ])));

    assert!(!Solution::is_palindrome(ListNode::from_slice(&[1, 2])));
    assert!(!Solution::is_palindrome(ListNode::from_slice(&[1, 2, 3])));
    assert!(!Solution::is_palindrome(ListNode::from_slice(&[
        1, 2, 2, 3
    ])));
    assert!(!Solution::is_palindrome(ListNode::from_slice(&[
        1, 2, 1, 2, 3
    ])));
}
