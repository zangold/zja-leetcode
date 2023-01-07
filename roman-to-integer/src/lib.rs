#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let v = s.chars().collect::<Vec<char>>();
        let mut b = &v[..];
        let mut value: i32 = 0;

        while !b.is_empty() {
            let n = b[0];

            let m = if b.len() == 1 { '?' } else { b[1] };

            value += match (n, m) {
                ('I', 'V') => {
                    b = &b[1..];
                    4
                }
                ('I', 'X') => {
                    b = &b[1..];
                    9
                }
                ('X', 'L') => {
                    b = &b[1..];
                    40
                }
                ('X', 'C') => {
                    b = &b[1..];
                    90
                }
                ('C', 'D') => {
                    b = &b[1..];
                    400
                }
                ('C', 'M') => {
                    b = &b[1..];
                    900
                }
                ('I', _) => 1,
                ('V', _) => 5,
                ('X', _) => 10,
                ('L', _) => 50,
                ('C', _) => 100,
                ('D', _) => 500,
                ('M', _) => 1000,
                (_, _) => panic!("invalid roman numeral"),
            };

            b = &b[1..];
        }

        value
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::roman_to_int("III".into()), 3);
    assert_eq!(Solution::roman_to_int("LVIII".into()), 58);
    assert_eq!(Solution::roman_to_int("MCMXCIV".into()), 1994);
}
