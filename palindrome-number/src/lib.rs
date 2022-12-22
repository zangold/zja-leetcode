#![allow(dead_code)]

struct Solution;

impl Solution {
    fn str_palindrome(mut x: &[char]) -> bool {
        while x.len() > 1 {
            if x[0] == x[x.len() - 1] {
                x = &x[1..x.len() - 1];
            } else {
                return false;
            }
        }

        true
    }

    pub fn is_palindrome(x: i32) -> bool {
        Solution::str_palindrome(&format!("{x}").chars().collect::<Vec<char>>()[..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        assert!(Solution::is_palindrome(121));
        assert!(!Solution::is_palindrome(-121));
        assert!(!Solution::is_palindrome(10));
    }
}
