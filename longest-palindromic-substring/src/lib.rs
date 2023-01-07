#![allow(dead_code)]

struct Solution;

impl Solution {
    const fn expand(sc: &[char], mut start: usize, mut end: usize) -> std::ops::Range<usize> {
        while start > 0 && end < sc.len() - 1 {
            if sc[start - 1] == sc[end + 1] {
                start -= 1;
                end += 1;
            } else {
                break;
            }
        }

        start..end + 1
    }

    pub fn longest_palindrome(s: String) -> String {
        let sc = s.chars().collect::<Vec<char>>();
        let mut longest = 0..1;

        let mut try_palindrome_from = |start, end| {
            if sc[start] != sc[end] {
                return;
            }

            let expanded = Self::expand(&sc[..], start, end);

            if expanded.len() > longest.len() {
                longest = expanded;
            }
        };

        for center in 0..sc.len() - 1 {
            try_palindrome_from(center, center);
            try_palindrome_from(center, center + 1);
        }

        sc[longest].iter().collect()
    }
}

#[test]
fn do_test() {
    assert_eq!(
        Solution::longest_palindrome("babad".into()),
        "bab".to_string()
    );
    assert_eq!(
        Solution::longest_palindrome("cbbd".into()),
        "bb".to_string()
    );
}
