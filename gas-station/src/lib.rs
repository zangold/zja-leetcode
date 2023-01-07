#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();
        let net = gas
            .into_iter()
            .zip(cost.into_iter())
            .map(|(g, c)| g - c)
            .collect::<Vec<_>>();

        let mut start = 0;
        let mut end = 0;
        let mut gas = net[start];
        let mut num_iters = 0;
        loop {
            while gas >= 0 {
                num_iters += 1;
                end = (end + 1) % n;
                gas += net[end];

                if end == start {
                    return start as i32;
                }
            }

            // traveled from start to end, but then ran out of gas.

            // If we've already iterated through the entire array, then there must be no solution.
            if num_iters > n {
                return -1;
            }

            num_iters += 1;
            start = (end + 1) % n;
            end = start;
            gas = net[start];
        }
    }
}

#[test]
fn do_test() {
    assert_eq!(
        Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
        3
    );
    assert_eq!(
        Solution::can_complete_circuit(vec![2, 3, 2], vec![3, 4, 3]),
        -1
    );
}
