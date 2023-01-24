// Problem description: https://leetcode.com/problems/decode-ways/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let s = s
            .into_bytes()
            .into_iter()
            .map(|c| c - b'0')
            .collect::<Vec<u8>>();

        let mut dp = vec![0; s.len() + 1];

        dp[0] = 1;

        for i in 1..dp.len() {
            if s[i - 1] != 0 {
                dp[i] += dp[i - 1];
            }

            if i >= 2 && s[i - 2] != 0 && (1..=26).contains(&(s[i - 2] * 10 + s[i - 1])) {
                dp[i] += dp[i - 2];
            }
        }

        dp[s.len()]
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::num_decodings("12".into()), 2);
    assert_eq!(Solution::num_decodings("226".into()), 3);
    assert_eq!(Solution::num_decodings("06".into()), 0);
}
