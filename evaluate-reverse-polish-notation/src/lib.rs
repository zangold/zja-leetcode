// Problem description: https://leetcode.com/problems/evaluate-reverse-polish-notation/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::<i32>::new();

        for token in tokens {
            match token.as_str() {
                "+" => {
                    let n = stack.pop().unwrap() + stack.pop().unwrap();
                    stack.push(n);
                }
                "-" => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b - a);
                }
                "*" => {
                    let n = stack.pop().unwrap() * stack.pop().unwrap();
                    stack.push(n);
                }
                "/" => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b / a);
                }
                num => stack.push(num.parse::<i32>().unwrap()),
            }
        }

        assert!(stack.len() == 1);

        stack[0]
    }
}

#[test]
fn do_test() {
    let ops = [
        "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
    ]
    .iter()
    .map(|w| w.to_string())
    .collect();

    assert_eq!(Solution::eval_rpn(ops), 22);

    let ops = ["2", "1", "+", "3", "*"]
        .iter()
        .map(|w| w.to_string())
        .collect();
    assert_eq!(Solution::eval_rpn(ops), 9);

    let ops = ["4", "13", "5", "/", "+"]
        .iter()
        .map(|w| w.to_string())
        .collect();
    assert_eq!(Solution::eval_rpn(ops), 6);
}
