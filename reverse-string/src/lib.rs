// https://leetcode.com/problems/reverse-string/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let l = s.len();
        (0..l / 2).for_each(|i| s.swap(i, l - i - 1))
    }
}

fn test_helper(s: &str) -> bool {
    let expected = s.chars().rev().collect::<Vec<char>>();
    let mut actual = s.chars().collect();
    Solution::reverse_string(&mut actual);

    expected == actual
}

#[test]
fn do_test() {
    assert!(test_helper("hello"));
    assert!(test_helper("Hannah"));
    assert!(test_helper("H"));
    assert!(test_helper(""));
    assert!(test_helper("this is another test case"));

    let mut s = "hello".chars().collect();
    Solution::reverse_string(&mut s);
    assert_eq!(s, "olleh".chars().collect::<Vec<char>>());

    let mut s = "Hannah".chars().collect();
    Solution::reverse_string(&mut s);
    assert_eq!(s, "hannaH".chars().collect::<Vec<char>>());
}
