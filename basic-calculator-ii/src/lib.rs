// Problem description: https://leetcode.com/problems/basic-calculator-ii/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut num = 0;
        let mut nums = vec![];
        let mut ops = vec![];

        for c in s.chars() {
            match c {
                '0'..='9' => {
                    num = num * 10 + c as i32 - '0' as i32;
                }
                '*' | '+' | '-' | '/' => {
                    nums.push(num);
                    num = 0;
                    ops.push(c);
                }
                ' ' => {}
                _ => panic!("invalid char in expression"),
            }
        }

        nums.push(num);

        let mut sum_nums = vec![nums[0]];

        for (index, num) in nums.iter().copied().enumerate().skip(1) {
            match ops[index - 1] {
                '*' => *sum_nums.last_mut().unwrap() *= num,
                '/' => *sum_nums.last_mut().unwrap() /= num,
                '-' => sum_nums.push(-num),
                '+' => sum_nums.push(num),
                _ => panic!("unexpected operator"),
            }
        }

        sum_nums.iter().sum::<i32>()
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::calculate("3+2*2".into()), 7);
    assert_eq!(Solution::calculate(" 3/2 ".into()), 1);
    assert_eq!(Solution::calculate(" 3+5 / 2 ".into()), 5);
    assert_eq!(Solution::calculate("0".into()), 0);
    assert_eq!(Solution::calculate("1 + 1".into()), 2);
    assert_eq!(Solution::calculate("2 * 2".into()), 4);
}
