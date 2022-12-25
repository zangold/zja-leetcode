#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn answer_queries(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut replies = vec![0i32; queries.len()];

        nums.sort_unstable();
        let mut sum: i32 = nums.iter().sum();

        let mut qs = queries.into_iter().enumerate().collect::<Vec<_>>();
        qs.sort_unstable_by_key(|(_i, q)| *q);

        while !nums.is_empty() {
            while !qs.is_empty() && qs.last().unwrap().1 >= sum {
                let q = qs.pop().unwrap();
                replies[q.0] = nums.len() as i32;
            }

            if qs.is_empty() {
                break;
            }

            sum -= nums.pop().unwrap();
        }

        replies
    }
}

#[test]
fn it_works() {
    assert_eq!(
        Solution::answer_queries(vec![4, 5, 2, 1], vec![3, 10, 21]),
        vec![2, 3, 4]
    );
    assert_eq!(Solution::answer_queries(vec![2, 3, 4, 5], vec![1]), vec![0]);
}
