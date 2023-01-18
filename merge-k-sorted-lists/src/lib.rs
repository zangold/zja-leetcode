// https://leetcode.com/problems/merge-k-sorted-lists/

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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut output: Option<Box<ListNode>> = None;
        let mut cursor: &mut Option<Box<ListNode>> = &mut output;

        // pre-filter the list-of-lists to remove any empty lists
        let mut lists = lists
            .into_iter()
            .filter(|list| list.is_some())
            .collect::<Vec<Option<_>>>();

        while lists.len() > 1 {
            // find the index of the node with the lowest value
            let index = lists
                .iter()
                .enumerate()
                .min_by_key(|(_, list)| {
                    list.as_ref()
                        .map_or_else(|| panic!("shouldn't get here"), |node| node.val)
                })
                .unwrap()
                .0;

            *cursor = lists[index].take();
            if let Some(ref mut node) = cursor {
                lists[index] = node.next.take();

                if lists[index].is_none() {
                    lists.remove(index);
                }

                cursor = &mut node.next;
            }
        }

        // small optimization for the last list
        if lists.len() == 1 {
            *cursor = lists[0].take();
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

    assert_eq!(Solution::merge_k_lists(vec![a, b]), c);

    let a = make_list(&[]);
    let b = make_list(&[]);
    let c = make_list(&[]);

    assert_eq!(Solution::merge_k_lists(vec![a, b]), c);

    let a = make_list(&[]);
    let b = make_list(&[0]);
    let c = make_list(&[0]);

    assert_eq!(Solution::merge_k_lists(vec![a, b]), c);

    let lists = vec![
        make_list(&[1, 4, 5]),
        make_list(&[1, 3, 4]),
        make_list(&[2, 6]),
    ];

    assert_eq!(
        Solution::merge_k_lists(lists),
        make_list(&[1, 1, 2, 3, 4, 4, 5, 6])
    );

    let lists = vec![];

    assert_eq!(Solution::merge_k_lists(lists), make_list(&[]));

    let lists = vec![make_list(&[])];

    assert_eq!(Solution::merge_k_lists(lists), make_list(&[]));

    let lists = vec![make_list(&[1, 2, 3])];

    assert_eq!(Solution::merge_k_lists(lists), make_list(&[1, 2, 3]));
}
