#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..=n)
            .into_iter()
            .map(|i| match i {
                i if i % 3 == 0 && i % 5 == 0 => "FizzBuzz".to_string(),
                i if i % 3 == 0 => "Fizz".to_string(),
                i if i % 5 == 0 => "Buzz".to_string(),
                i => format!("{i}"),
            })
            .collect()
    }
}

#[test]
fn do_test() {
    assert_eq!(
        Solution::fizz_buzz(15),
        vec![
            "1".to_string(),
            "2".to_string(),
            "Fizz".to_string(),
            "4".to_string(),
            "Buzz".to_string(),
            "Fizz".to_string(),
            "7".to_string(),
            "8".to_string(),
            "Fizz".to_string(),
            "Buzz".to_string(),
            "11".to_string(),
            "Fizz".to_string(),
            "13".to_string(),
            "14".to_string(),
            "FizzBuzz".to_string(),
        ]
    )
}
