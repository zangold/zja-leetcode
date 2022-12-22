#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn longest_common_prefix(mut strs: Vec<String>) -> String {
        let mut prefix = String::new();

        loop {
            let mut c: Option<char> = None;

            for str in &mut strs[..] {
                if str.is_empty() {
                    return prefix;
                }

                if let Some(ch) = c {
                    if ch != str.chars().next().unwrap() {
                        return prefix;
                    }
                } else {
                    c = Some(str.chars().next().unwrap());
                }

                str.remove(0);
            }

            prefix.push(c.unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_helper(strs: Vec<&str>, expected: &str) {
        assert_eq!(
            Solution::longest_common_prefix(strs.into_iter().map(|x| x.to_string()).collect()),
            expected.to_string()
        )
    }

    #[test]
    fn main() {
        test_helper(vec!["flower", "flow", "flight"], "fl");
        test_helper(vec!["dog", "racecar", "car"], "");
    }
}
