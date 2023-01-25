// Problem description: https://leetcode.com/problems/count-and-say/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut s = "1".to_string();

        for _ in 1..n {
            let mut new_s = String::new();

            let mut it = s.chars();
            let mut last_char = it.next().unwrap();
            let mut last_char_count = 1;
            for c in it {
                if c == last_char {
                    last_char_count += 1;
                } else {
                    new_s.push_str(format!("{last_char_count}{last_char}").as_str());
                    last_char = c;
                    last_char_count = 1;
                }
            }

            new_s.push_str(format!("{last_char_count}{last_char}").as_str());

            s = new_s;
        }

        s
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::count_and_say(1), "1");
    assert_eq!(Solution::count_and_say(2), "11");
    assert_eq!(Solution::count_and_say(3), "21");
    assert_eq!(Solution::count_and_say(4), "1211");
    assert_eq!(Solution::count_and_say(5), "111221");
    assert_eq!(Solution::count_and_say(6), "312211");
    assert_eq!(Solution::count_and_say(7), "13112221");
    assert_eq!(Solution::count_and_say(8), "1113213211");
    assert_eq!(Solution::count_and_say(9), "31131211131221");
}
