// https://leetcode.com/problems/minimum-rounds-to-complete-all-tasks/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn minimum_rounds(mut tasks: Vec<i32>) -> i32 {
        tasks.sort_unstable();

        // .0 is the task difficulty, .1 is the number of tasks at that difficulty
        let mut binned_tasks = Vec::<(i32, i32)>::new();

        tasks.iter().for_each(|t| match binned_tasks.last_mut() {
            Some(task) if task.0 == *t => task.1 += 1,
            _ => binned_tasks.push((*t, 1)),
        });

        let mut num_rounds = 0;

        for (_, count) in binned_tasks.into_iter() {
            num_rounds += match count {
                1 => return -1,
                n if n % 3 == 0 => n / 3,
                n => n / 3 + 1,
            }
        }

        num_rounds
    }
}

#[test]
fn do_test() {
    assert_eq!(
        Solution::minimum_rounds(vec![2, 2, 3, 3, 2, 4, 4, 4, 4, 4]),
        4
    );
    assert_eq!(Solution::minimum_rounds(vec![2, 3, 3]), -1);

    assert_eq!(Solution::minimum_rounds(vec![5; 1]), -1);
    assert_eq!(Solution::minimum_rounds(vec![5; 2]), 1);
    assert_eq!(Solution::minimum_rounds(vec![5; 3]), 1);
    assert_eq!(Solution::minimum_rounds(vec![5; 4]), 2);
    assert_eq!(Solution::minimum_rounds(vec![5; 5]), 2);
    assert_eq!(Solution::minimum_rounds(vec![5; 6]), 2);
    assert_eq!(Solution::minimum_rounds(vec![5; 7]), 3);
    assert_eq!(Solution::minimum_rounds(vec![5; 8]), 3);
    assert_eq!(Solution::minimum_rounds(vec![5; 9]), 3);
}
