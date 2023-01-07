#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut digit = 1;

        while digit <= digits.len() {
            let index = digits.len() - digit;
            digits[index] = (digits[index] + 1) % 10;

            if digits[index] != 0 {
                break;
            }

            digit += 1;
        }

        if digit == digits.len() + 1 {
            digits.insert(0, 1);
        }

        digits
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
    assert_eq!(Solution::plus_one(vec![9]), vec![1, 0]);
    assert_eq!(Solution::plus_one(vec![9, 9, 9]), vec![1, 0, 0, 0]);
}
