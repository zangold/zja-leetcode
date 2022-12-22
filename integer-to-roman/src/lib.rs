#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let mut roman = String::new();

        while num > 0 {
            let (value, chars) = match num {
                n if n >= 1000 => (1000, "M"),
                n if n >= 900 => (900, "CM"),
                n if n >= 500 => (500, "D"),
                n if n >= 400 => (400, "CD"),
                n if n >= 100 => (100, "C"),
                n if n >= 90 => (90, "XC"),
                n if n >= 50 => (50, "L"),
                n if n >= 40 => (40, "XL"),
                n if n >= 10 => (10, "X"),
                n if n >= 9 => (9, "IX"),
                n if n >= 5 => (5, "V"),
                n if n >= 4 => (4, "IV"),
                _ => (1, "I"),
            };

            roman.push_str(chars);
            num -= value;
        }

        roman
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roman_to_int(s: String) -> i32 {
        let v = s.chars().collect::<Vec<char>>();
        let mut b = &v[..];
        let mut value: i32 = 0;

        while b.len() > 0 {
            let n = b[0];

            let m = if b.len() == 1 {
                '?'
            } else {
                b[1]
            };

            value += match (n, m) {
                ('I', 'V') => { b = &b[1..]; 4 },
                ('I', 'X') => { b = &b[1..]; 9 },
                ('X', 'L') => { b = &b[1..]; 40 },
                ('X', 'C') => { b = &b[1..]; 90 },
                ('C', 'D') => { b = &b[1..]; 400 },
                ('C', 'M') => { b = &b[1..]; 900 },
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

    fn test_case(num: i32, expected: &str) {
        assert_eq!(Solution::int_to_roman(num), expected.to_string())
    }

    #[test]
    fn do_tests() {
        test_case(3, "III");
        test_case(58, "LVIII");
        test_case(1994, "MCMXCIV");

        test_case(899, "DCCCXCIX");

        for i in 1..4000 {
            assert_eq!(roman_to_int(Solution::int_to_roman(i)), i);
        }
    }
}
