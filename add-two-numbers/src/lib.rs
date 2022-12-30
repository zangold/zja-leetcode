#![allow(dead_code)]

struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<Self>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        Self { next: None, val }
    }

    fn new_next(val: i32, next: Option<Box<Self>>) -> Self {
        Self { next, val }
    }
}

impl Solution {
    fn add_helper(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        carry: i32,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(n1), Some(n2)) => {
                let sum = n1.val + n2.val + carry;
                Some(Box::new(ListNode {
                    val: sum % 10,
                    next: Self::add_helper(n1.next, n2.next, sum / 10),
                }))
            }
            (Some(n), None) | (None, Some(n)) => {
                let sum = n.val + carry;
                Some(Box::new(ListNode {
                    val: sum % 10,
                    next: Self::add_helper(n.next, None, sum / 10),
                }))
            }
            (None, None) if carry != 0 => Some(Box::new(ListNode::new(carry))),
            (None, None) => None,
        }
    }

    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::add_helper(l1, l2, 0)
    }
}

fn list_from_slice(digits: &[i32]) -> Option<Box<ListNode>> {
    match digits.len() {
        0 => None,
        1 => Some(Box::new(ListNode::new(digits[0]))),
        _ => Some(Box::new(ListNode::new_next(
            digits[0],
            list_from_slice(&digits[1..]),
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn do_tests() {
        let a = list_from_slice(&[2, 4, 3]);
        let b = list_from_slice(&[5, 6, 4]);

        println!("{:?}", Solution::add_two_numbers(a, b));

        let a = Some(Box::new(ListNode::new(0)));
        let b = Some(Box::new(ListNode::new(0)));

        println!("{:?}", Solution::add_two_numbers(a, b));

        let a = list_from_slice(&[9, 9, 9, 9, 9, 9, 9]);
        let b = list_from_slice(&[9, 9, 9, 9]);

        println!("{:?}", Solution::add_two_numbers(a, b));

        let a = list_from_slice(&[2, 4, 9]);
        let b = list_from_slice(&[5, 6, 4, 9]);

        println!("{:?}", Solution::add_two_numbers(a, b));
    }
}
