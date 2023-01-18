// https://leetcode.com/problems/number-of-nodes-in-the-sub-tree-with-the-same-label/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
        let labels = labels.chars().collect::<Vec<char>>();
        let n = n as usize;

        let mut new_edges = vec![Vec::<usize>::new(); n];
        for edge in edges {
            new_edges[edge[0] as usize].push(edge[1] as usize);
            new_edges[edge[1] as usize].push(edge[0] as usize);
        }

        let mut label_counts = vec![[0; 26]; n];

        fn compute_label_counts(
            label_counts: &mut [[usize; 26]],
            edges: &[Vec<usize>],
            labels: &[char],
            node: usize,
            parent: usize,
        ) {
            for edge in edges[node].iter().copied() {
                if edge == parent {
                    continue;
                }

                compute_label_counts(label_counts, edges, labels, edge, node);

                for i in 0..26 {
                    label_counts[node][i] += label_counts[edge][i];
                }
            }

            label_counts[node][labels[node] as usize - b'a' as usize] += 1;
        }

        compute_label_counts(&mut label_counts, &new_edges, &labels, 0, usize::MAX);

        (0..n)
            .into_iter()
            .map(|i| label_counts[i][labels[i] as usize - b'a' as usize] as i32)
            .collect()
    }
}

#[test]
fn do_test_1() {
    let n = 7;
    let edges = vec![
        vec![0, 1],
        vec![0, 2],
        vec![1, 4],
        vec![1, 5],
        vec![2, 3],
        vec![2, 6],
    ];

    let labels = "abaedcd".to_string();
    let expected = vec![2, 1, 1, 1, 1, 1, 1];

    assert_eq!(Solution::count_sub_trees(n, edges, labels), expected);
}

#[test]
fn do_test_2() {
    let n = 4;
    let edges = vec![vec![0, 1], vec![1, 2], vec![0, 3]];

    let labels = "bbbb".to_string();
    let expected = vec![4, 2, 1, 1];

    assert_eq!(Solution::count_sub_trees(n, edges, labels), expected);
}

#[test]
fn do_test_3() {
    let n = 5;
    let edges = vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![0, 4]];

    let labels = "aabab".to_string();
    let expected = vec![3, 2, 1, 1, 1];

    assert_eq!(Solution::count_sub_trees(n, edges, labels), expected);
}

#[test]
fn do_test_4() {
    let n = 5;
    let edges = vec![vec![0, 3], vec![0, 2], vec![3, 1], vec![0, 4]];

    let labels = "aabab".to_string();
    let expected = vec![3, 1, 1, 2, 1];

    assert_eq!(Solution::count_sub_trees(n, edges, labels), expected);
}
