#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut longest = (0, 0);

        let mut cursor = (0, 0);
        let mut present: [bool; 256] = [false; 256];

        let char_index = |c| c as usize;

        loop {
            // First, expand the substring to the right until just before we get two of the same
            // character in the substring, or we reach the end of the string.
            while cursor.1 != s.len() {
                if !present[char_index(s.as_bytes()[cursor.1])] {
                    present[char_index(s.as_bytes()[cursor.1])] = true;
                } else {
                    break;
                }

                cursor.1 += 1;
            }

            // Then, possibly record this substring as the longest. If we're at the end of the
            // string, we can break here.
            if (longest.1 - longest.0) < (cursor.1 - cursor.0) {
                longest = cursor;
            }

            if cursor.1 == s.len() {
                break;
            }

            let next_char = s.as_bytes()[cursor.1];

            // Finally, advance the left end of the substring cursor until we no longer have the
            // duplicated character, or we reach the right end of the substring cursor.
            while cursor.0 < cursor.1 {
                let c = s.as_bytes()[cursor.0];

                present[char_index(c)] = false;
                cursor.0 += 1;

                if c == next_char {
                    break;
                }
            }
        }

        longest.1 as i32 - longest.0 as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn do_tests() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring("".to_string()), 0);
    }
}
