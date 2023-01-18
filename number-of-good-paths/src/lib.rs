// https://leetcode.com/problems/number-of-good-paths/

#![allow(dead_code)]

struct Solution;

use std::collections::{BTreeMap, HashMap};

impl Solution {
    pub fn number_of_good_paths(vals: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let mut new_edges = vec![vec![]; vals.len()];

        for edge in edges {
            new_edges[edge[0] as usize].push(edge[1]);
            new_edges[edge[1] as usize].push(edge[0]);
        }

        let mut nodes_by_value = BTreeMap::<i32, Vec<i32>>::new();

        for (index, val) in vals.iter().enumerate() {
            if let Some(nodes) = nodes_by_value.get_mut(val) {
                nodes.push(index as i32);
            } else {
                nodes_by_value.insert(*val, vec![index as i32]);
            }
        }

        let nodes_by_value = nodes_by_value
            .into_iter()
            .map(|(_, v)| v)
            .collect::<Vec<Vec<i32>>>();

        // Disjoint sets
        let mut parent = (0..vals.len() as i32).into_iter().collect::<Vec<_>>();

        fn find(parent: &mut Vec<i32>, a: i32) -> i32 {
            if a != parent[a as usize] {
                parent[a as usize] = find(parent, parent[a as usize]);
            }

            parent[a as usize]
        }

        fn union(parent: &mut Vec<i32>, a: i32, b: i32) {
            let sa = find(parent, a);
            let sb = find(parent, b);

            parent[sa as usize] = sb;
        }

        let mut good_paths_count = 0;

        for nodes in nodes_by_value.iter() {
            // Add each of the nodes' (valid) neighbours into its set
            for node in nodes.iter().copied() {
                for edge in new_edges[node as usize].iter() {
                    if vals[node as usize] >= vals[*edge as usize] {
                        union(&mut parent, node, *edge);
                    }
                }
            }

            // <set representative, set count>
            let mut set_counts = HashMap::<i32, i32>::new();
            for node in nodes.iter().copied() {
                let set_rep = find(&mut parent, node);
                if let Some(count) = set_counts.get_mut(&set_rep) {
                    *count += 1;
                } else {
                    set_counts.insert(set_rep, 1);
                }
            }

            for (_, set_count) in set_counts.iter() {
                good_paths_count += set_count * (set_count + 1) / 2;
            }
        }

        good_paths_count as i32
    }
}

#[test]
fn do_test_1() {
    let vals = vec![1, 3, 2, 1, 3];
    let edges = vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]];
    assert_eq!(Solution::number_of_good_paths(vals, edges), 6);
}

#[test]
fn do_test_2() {
    let vals = vec![1, 1, 2, 2, 3];
    let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![2, 4]];
    assert_eq!(Solution::number_of_good_paths(vals, edges), 7);
}

#[test]
fn do_test_3() {
    let vals = vec![1];
    let edges = vec![];
    assert_eq!(Solution::number_of_good_paths(vals, edges), 1);
}
