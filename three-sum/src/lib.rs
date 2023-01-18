// https://leetcode.com/problems/three-sum/

#![allow(dead_code)]

struct Solution;

impl Solution {
    fn contains(haystack: &[i32], needle: i32) -> bool {
        if haystack.is_empty() {
            return false;
        }

        match haystack[haystack.len() / 2] {
            value if value == needle => true,
            value if value < needle => Self::contains(&haystack[haystack.len() / 2 + 1..], needle),
            _ => Self::contains(&haystack[..haystack.len() / 2], needle),
        }
    }

    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut results = Vec::<Vec<i32>>::new();

        nums.sort_unstable();

        // Select the first element, indexed by 'a', and then scan nums to find pairs (b,c) such
        // that nums[b] + nums[c] == -nums[a].
        for a in 0..nums.len() - 2 {
            // Skip same values for 'a'
            if a > 0 && nums[a] == nums[a - 1] {
                continue;
            }

            for b in a + 1..nums.len() - 1 {
                // Skip same values for 'b'
                if b > a + 1 && nums[b] == nums[b - 1] {
                    continue;
                }

                if Self::contains(&nums[b + 1..nums.len()], -nums[a] - nums[b]) {
                    results.push(vec![nums[a], nums[b], -nums[a] - nums[b]]);
                }
            }
        }

        results
    }
}

#[test]
fn do_test() {
    assert_eq!(
        Solution::three_sum(vec![-1, 0, 1, 2, -1, 4]),
        vec![vec![-1, -1, 2], vec![-1, 0, 1]]
    );
    assert_eq!(Solution::three_sum(vec![0, 1, 1]), Vec::<Vec<i32>>::new());
    assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);

    assert_eq!(
        Solution::three_sum(vec![-1, -1, 0, 0, 1, 1]),
        vec![vec![-1, 0, 1]]
    );
}
