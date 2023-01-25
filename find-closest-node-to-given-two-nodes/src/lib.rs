// Problem description: https://leetcode.com/problems/find-closest-node-to-given-two-nodes/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        let n = edges.len();

        let mut dist1 = vec![-1; n];
        let mut dist2 = vec![-1; n];

        let mut cursor = node1;
        let mut steps = 0;
        while cursor != -1 && dist1[cursor as usize] == -1 {
            dist1[cursor as usize] = steps;
            steps += 1;
            cursor = edges[cursor as usize];
        }

        cursor = node2;
        steps = 0;
        while cursor != -1 && dist2[cursor as usize] == -1 {
            dist2[cursor as usize] = steps;
            steps += 1;
            cursor = edges[cursor as usize];
        }

        let mut best_max_dist = i32::MAX;
        let mut best_node = -1;

        for i in 0..n {
            if dist1[i] == -1 || dist2[i] == -1 {
                continue;
            }

            if i32::max(dist1[i], dist2[i]) < best_max_dist {
                best_max_dist = i32::max(dist1[i], dist2[i]);
                best_node = i as i32;
            }
        }

        best_node
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::closest_meeting_node(vec![2, 2, 3, -1], 0, 1), 2);
    assert_eq!(Solution::closest_meeting_node(vec![1, 2, -1], 0, 2), 2);
}
