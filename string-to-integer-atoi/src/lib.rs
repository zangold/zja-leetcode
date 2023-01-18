// https://leetcode.com/problems/string-to-integer-atoi/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut it = s.chars();
        let mut result: i64 = 0;
        let mut negative = false;

        // step 1: skip leading whitespace
        loop {
            match it.next() {
                Some('+') => break,
                Some('-') => {
                    negative = true;
                    break;
                }
                Some(c) if ('0'..='9').contains(&c) => {
                    result = c as i64 - '0' as i64;
                    break;
                }
                Some(' ') => {}
                None | Some(_) => return 0,
            }
        }

        loop {
            match it.next() {
                Some(d) if ('0'..='9').contains(&d) => {
                    let digit = d as i64 - '0' as i64;
                    result = result * 10 + digit;

                    if negative && result > (1 << 31) {
                        return i32::MIN;
                    } else if !negative && result > (1 << 31) - 1 {
                        return i32::MAX;
                    }
                }
                Some(_) | None => break,
            }
        }

        (if negative { -result } else { result }) as i32
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::my_atoi("42".into()), 42);
    assert_eq!(Solution::my_atoi("-42".into()), -42);
    assert_eq!(Solution::my_atoi("4193 with words".into()), 4193);
    assert_eq!(Solution::my_atoi("  +4193 with words".into()), 4193);
    assert_eq!(Solution::my_atoi("  -4193 with words".into()), -4193);

    assert_eq!(
        Solution::my_atoi("  +10000000000 with words".into()),
        i32::MAX
    );
    assert_eq!(
        Solution::my_atoi("  -10000000000 with words".into()),
        i32::MIN
    );

    assert_eq!(
        Solution::my_atoi("  -2147483648 with words".into()),
        i32::MIN
    );
    assert_eq!(
        Solution::my_atoi("  +2147483647 with words".into()),
        i32::MAX
    );

    assert_eq!(
        Solution::my_atoi("  -2147483649 with words".into()),
        i32::MIN
    );
    assert_eq!(
        Solution::my_atoi("  +2147483648 with words".into()),
        i32::MAX
    );

    assert_eq!(
        Solution::my_atoi("  -2147483647 with words".into()),
        -2147483647
    );
    assert_eq!(
        Solution::my_atoi("  +2147483646 with words".into()),
        2147483646
    );

    assert_eq!(Solution::my_atoi("words and 987".into()), 0);
}
