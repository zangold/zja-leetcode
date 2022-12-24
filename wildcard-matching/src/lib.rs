#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.chars().collect::<Vec<char>>();
        let p = p.chars().collect::<Vec<char>>();
        let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];

        dp[0][0] = true;

        for j in 1..=p.len() {
            if p[j - 1] == '*' {
                let mut matching = false;

                for line in dp.iter_mut() {
                    if line[j - 1] {
                        matching = true;
                    }

                    line[j] = matching;
                }
            }

            for i in 1..=s.len() {
                match (s[i - 1], p[j - 1]) {
                    (x, y) if x == y || y == '?' => {
                        dp[i][j] = dp[i - 1][j - 1];
                    }
                    (_, _) => {}
                }
            }
        }

        dp[s.len()][p.len()]
    }
}

#[test]
fn do_test() {
    assert!(!Solution::is_match("aa".into(), "a".into()));
    assert!(Solution::is_match("aa".into(), "*".into()));
    assert!(!Solution::is_match("cb".into(), "?a".into()));
    assert!(Solution::is_match("cb".into(), "?b".into()));
    assert!(Solution::is_match("cb".into(), "c?".into()));
    assert!(Solution::is_match("cb".into(), "??".into()));
    assert!(!Solution::is_match("cb".into(), "???".into()));
    assert!(Solution::is_match("aaabbbaaa".into(), "a*ab?b*a".into()));
    assert!(!Solution::is_match("aaabbbaaa".into(), "a*ab?x*a".into()));
    assert!(!Solution::is_match("aaabbbaaa".into(), "a*ab??b*a".into()));
    assert!(Solution::is_match("adceb".into(), "*a*b".into()));
}
