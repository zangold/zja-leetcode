#![allow(dead_code)]

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
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k < 2 {
            return head;
        }

        let mut values = vec![];
        let k = k as usize;

        while let Some(node) = head {
            values.push(node.val);
            head = node.next;
        }

        let mut i = 0;
        while i + k <= values.len() {
            for j in 0..k / 2 {
                values.swap(i + j, i + k - j - 1);
            }

            i += k;
        }

        make_list(&values)
    }
}

fn make_list(values: &[i32]) -> Option<Box<ListNode>> {
    if values.is_empty() {
        None
    } else {
        Some(Box::new(ListNode {
            val: values[0],
            next: make_list(&values[1..]),
        }))
    }
}

#[test]
fn do_test() {
    let list = make_list(&[1, 2, 3, 4, 5]);
    let expected = make_list(&[2, 1, 4, 3, 5]);

    assert_eq!(Solution::reverse_k_group(list, 2), expected);

    let list = make_list(&[1, 2, 3, 4, 5]);
    let expected = make_list(&[3, 2, 1, 4, 5]);

    assert_eq!(Solution::reverse_k_group(list, 3), expected);

    let list = make_list(&[1, 2, 3, 4, 5, 6]);
    let expected = make_list(&[3, 2, 1, 6, 5, 4]);

    assert_eq!(Solution::reverse_k_group(list, 3), expected);

    let list = make_list(&[1, 2]);
    let expected = make_list(&[1, 2]);

    assert_eq!(Solution::reverse_k_group(list, 3), expected);

    let list = make_list(&[1, 2, 3, 4, 5, 6]);
    let expected = make_list(&[1, 2, 3, 4, 5, 6]);

    assert_eq!(Solution::reverse_k_group(list, 1), expected);
}
