#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut result: i32 = 0;

        let char_to_int = |c: char| -> i32 { (c as u8 - b'A') as i32 + 1 };

        for c in column_title.chars() {
            result *= 26;
            result += char_to_int(c);
        }

        result
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::title_to_number("A".into()), 1);
    assert_eq!(Solution::title_to_number("AB".into()), 28);
    assert_eq!(Solution::title_to_number("ZY".into()), 701);
    assert_eq!(Solution::title_to_number("FXSHRXW".into()), i32::MAX);
}
