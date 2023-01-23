// Problem description: https://leetcode.com/problems/find-the-town-judge/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut trusts = vec![0; n];
        let mut trusted_by = vec![0; n];

        for t in trust {
            trusts[t[0] as usize - 1] += 1;
            trusted_by[t[1] as usize - 1] += 1;
        }

        for i in 0..n {
            if trusts[i] == 0 && trusted_by[i] == n - 1 {
                return i as i32 + 1;
            }
        }

        -1
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::find_judge(2, vec![vec![1, 2]]), 2);
    assert_eq!(Solution::find_judge(3, vec![vec![1, 3], vec![2, 3]]), 3);
    assert_eq!(
        Solution::find_judge(3, vec![vec![1, 3], vec![2, 3], vec![3, 1]]),
        -1
    );
}
