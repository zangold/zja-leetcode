#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
        let mut new_edges: Vec<Vec<i32>> = vec![Vec::<i32>::new(); n as usize];

        for e in edges.into_iter() {
            new_edges[e[0] as usize].push(e[1]);
            new_edges[e[1] as usize].push(e[0]);
        }

        // return is: Some(steps), or None if the subtree wasn't worth visiting.
        fn num_steps(
            parent: usize,
            start: usize,
            edges: &[Vec<i32>],
            has_apple: &[bool],
        ) -> Option<i32> {
            let mut steps = 0;
            let mut subtree_has_apples = has_apple[start];

            for edge in edges[start].iter().copied() {
                if edge == parent as i32 {
                    continue;
                }

                if let Some(sub_steps) = num_steps(start, edge as usize, edges, has_apple) {
                    steps += sub_steps + 2;
                    subtree_has_apples = true;
                }
            }

            if subtree_has_apples {
                Some(steps)
            } else {
                None
            }
        }

        num_steps(usize::MAX, 0, &new_edges, &has_apple).unwrap_or(0)
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

    let has_apple = vec![false, false, true, false, true, true, false];

    assert_eq!(Solution::min_time(n, edges, has_apple), 8);
}

#[test]
fn do_test_2() {
    let n = 7;

    let edges = vec![
        vec![0, 1],
        vec![0, 2],
        vec![1, 4],
        vec![1, 5],
        vec![2, 3],
        vec![2, 6],
    ];

    let has_apple = vec![false, false, true, false, false, true, false];

    assert_eq!(Solution::min_time(n, edges, has_apple), 6);
}

#[test]
fn do_test_3() {
    let n = 7;

    let edges = vec![
        vec![0, 1],
        vec![0, 2],
        vec![1, 4],
        vec![1, 5],
        vec![2, 3],
        vec![2, 6],
    ];

    let has_apple = vec![false; 7];

    assert_eq!(Solution::min_time(n, edges, has_apple), 0);
}

#[test]
fn do_test_4() {
    let n = 4;

    let edges = vec![vec![0, 2], vec![0, 3], vec![1, 2]];

    let has_apple = vec![false, true, false, false];

    assert_eq!(Solution::min_time(n, edges, has_apple), 4);
}
