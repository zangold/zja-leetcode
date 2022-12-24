#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut reversed: u32 = 0;

        let sign: i32 = if x < 0 { -1 } else { 1 };
        let mut x: u32 = (x * sign) as u32;

        while x != 0 {
            if (i32::MAX as u32 - (x % 10)) / 10 < reversed {
                return 0;
            }

            reversed = reversed * 10 + (x % 10);
            x /= 10;
        }

        reversed as i32 * sign
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::reverse(123), 321);
    assert_eq!(Solution::reverse(-123), -321);
    assert_eq!(Solution::reverse(120), 21);

    assert_eq!(Solution::reverse(1563847412), 0);
    assert_eq!(Solution::reverse(1463847412), 2147483641);

    assert_eq!(Solution::reverse(-1463847412), -2147483641);
    assert_eq!(Solution::reverse(-1563847412), 0);
}
