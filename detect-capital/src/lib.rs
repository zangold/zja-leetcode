// https://leetcode.com/problems/detect-capital/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut char_iter = word.chars();
        let expect_upper: bool;

        match (char_iter.next(), char_iter.next()) {
            (Some(c1), Some(c2)) => match (c1.is_ascii_uppercase(), c2.is_ascii_uppercase()) {
                (false, false) => expect_upper = false,
                (true, false) => expect_upper = false,
                (false, true) => return false,
                (true, true) => expect_upper = true,
            },
            (_, _) => return true,
        }

        char_iter.all(|c| c.is_ascii_uppercase() == expect_upper)
    }
}

#[test]
fn do_test() {
    assert!(Solution::detect_capital_use("USA".into()));
    assert!(!Solution::detect_capital_use("FlaG".into()));
    assert!(Solution::detect_capital_use("Flag".into()));
    assert!(Solution::detect_capital_use("flag".into()));

    assert!(Solution::detect_capital_use("".into()));
    assert!(Solution::detect_capital_use("f".into()));
    assert!(Solution::detect_capital_use("F".into()));

    assert!(Solution::detect_capital_use("Fy".into()));
    assert!(Solution::detect_capital_use("fy".into()));
    assert!(!Solution::detect_capital_use("fY".into()));
    assert!(Solution::detect_capital_use("FY".into()));

    assert!(Solution::detect_capital_use("Foobar".into()));
    assert!(Solution::detect_capital_use("foobar".into()));
    assert!(!Solution::detect_capital_use("fooBAR".into()));
    assert!(Solution::detect_capital_use("FOOBAR".into()));
}
