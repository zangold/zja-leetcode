#![allow(dead_code)]

struct Solution;

impl Solution {
    fn helper(children: &[Vec<usize>], node: usize, s: &[char]) -> (usize, usize) {
        // For each child:
        // - compute the longest path in that subtree
        // - compute the longest path in that subtree with an end at the subtree root

        // Then:
        // - the longest path in this subtree is the sum of its two largest (labels permitting)
        //   subtrees-ending-at-root, plus one.
        // - unless there's a child with a longer contained path, in which case it's that.
        // - the longest path in this subtree ending at the root is one more than the longest
        //   subtree-ending-at-root, label permitting.

        if children[node].is_empty() {
            return (1, 1);
        }

        let mut longest_in_subtree_end_root = Vec::<usize>::with_capacity(children[node].len());

        let mut longest_in_tree = 0;
        let mut longest_in_tree_end_root = 1;

        for c in children[node].iter().copied() {
            let (a, b) = Self::helper(children, c, s);

            longest_in_tree = longest_in_tree.max(a);

            // filter invalid paths (same character) here
            if s[node] != s[c] {
                longest_in_subtree_end_root.push(b);
                longest_in_tree_end_root = longest_in_tree_end_root.max(b + 1);
            }
        }

        // make the longest not-ending-at-root path possible using the top two elements of
        // longest_in_subtree_end_root
        if longest_in_subtree_end_root.len() >= 2 {
            longest_in_subtree_end_root.sort_unstable();
            let mut it = longest_in_subtree_end_root.iter().rev();
            let a = *it.next().unwrap();
            let b = *it.next().unwrap();

            longest_in_tree = longest_in_tree.max(a + b + 1);
        }

        longest_in_tree = longest_in_tree.max(longest_in_tree_end_root);

        (longest_in_tree, longest_in_tree_end_root)
    }

    pub fn longest_path(parent: Vec<i32>, s: String) -> i32 {
        let mut children = vec![vec![0usize; 0]; parent.len()];
        let s = s.chars().collect::<Vec<char>>();

        parent
            .into_iter()
            .enumerate()
            .skip(1)
            .for_each(|(i, p)| children[p as usize].push(i));

        Self::helper(&children, 0, &s).0 as i32
    }
}

#[test]
fn do_test() {
    let parent = vec![-1, 0, 0, 1, 1, 2];
    let s = "abacbe".to_string();

    assert_eq!(Solution::longest_path(parent, s), 3);

    let parent = vec![-1, 0, 0, 0];
    let s = "aabc".to_string();

    assert_eq!(Solution::longest_path(parent, s), 3);

    let parent = vec![-1, 0, 1];
    let s = "aab".to_string();

    assert_eq!(Solution::longest_path(parent, s), 2);
}
