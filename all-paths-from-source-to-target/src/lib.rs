#![allow(dead_code)]

struct Solution;

impl Solution {
    fn helper(graph: &[Vec<i32>], start: usize, path: &mut Vec<i32>, paths: &mut Vec<Vec<i32>>) {
        path.push(start as i32);
        if start == graph.len() - 1 {
            paths.push(path.clone());
            path.pop();
            return;
        }

        for next in graph[start].iter() {
            Solution::helper(graph, *next as usize, path, paths);
        }

        path.pop();
    }

    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut paths: Vec<Vec<i32>> = Default::default();
        let mut path = Vec::<i32>::new();
        Solution::helper(&graph, 0, &mut path, &mut paths);
        paths
    }
}

#[test]
fn do_test() {
    let graph = vec![vec![1, 2], vec![3], vec![3], vec![]];
    let expected = vec![vec![0, 1, 3], vec![0, 2, 3]];
    assert_eq!(Solution::all_paths_source_target(graph), expected);

    let graph = vec![vec![4, 3, 1], vec![3, 2, 4], vec![3], vec![4], vec![]];
    let expected = vec![
        vec![0, 4],
        vec![0, 3, 4],
        vec![0, 1, 3, 4],
        vec![0, 1, 2, 3, 4],
        vec![0, 1, 4],
    ];
    assert_eq!(Solution::all_paths_source_target(graph), expected);
}
