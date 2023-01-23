// Problem description: https://leetcode.com/problems/course-schedule/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;
        let mut can_finish = vec![false; num_courses];
        let mut new_pr = vec![vec![]; num_courses];

        for pr in prerequisites {
            new_pr[pr[0] as usize].push(pr[1]);
        }

        let mut progress = true;
        while progress {
            progress = false;
            for i in 0..num_courses {
                if can_finish[i] {
                    continue;
                }

                if new_pr[i].iter().all(|i| can_finish[*i as usize]) {
                    progress = true;
                    can_finish[i] = true;
                }
            }
        }

        can_finish.into_iter().all(|b| b)
    }
}

#[test]
fn do_test() {
    assert!(Solution::can_finish(2, vec![vec![1, 0]]));
    assert!(!Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]));
    assert!(Solution::can_finish(2, vec![]));
    assert!(!Solution::can_finish(
        3,
        vec![vec![0, 1], vec![1, 2], vec![2, 0]]
    ));
}
