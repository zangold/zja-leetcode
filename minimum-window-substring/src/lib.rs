// https://leetcode.com/problems/minimum-window-substring/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let s = s.chars().collect::<Vec<char>>();

        let char_index = |c: char| {
            if ('a'..='z').contains(&c) {
                c as usize - b'a' as usize
            } else if ('A'..='Z').contains(&c) {
                c as usize - b'A' as usize + 26
            } else {
                panic!("shouldn't be able to get here");
            }
        };

        // Do some preprocessing first on t. Convert it into an array [usize, 52] of occurrence
        // counts for lower / upper case letters.
        let mut tt = [0; 52];
        t.chars().for_each(|c| tt[char_index(c)] += 1);

        // tracks the number of occurrences of each character in our current window
        let mut st = [0; 52];

        let mut window = 0..0;
        let mut minimum_window: Option<std::ops::Range<usize>> = None;

        loop {
            // extend the window to the right until each element of st is larger than the
            // corresponding element of tt (meaning that this is a valid window).
            loop {
                // If we hit the end of the string at this point, just return.
                if window.end == s.len() {
                    if let Some(w) = minimum_window {
                        return s[w].iter().collect();
                    } else {
                        return "".to_string();
                    }
                }

                window.end += 1;
                let ci = char_index(s[window.end - 1]);
                st[ci] += 1;

                // Check if this is now a valid window.
                if st[ci] == tt[ci] && (0..52).all(|i| st[i] >= tt[i]) {
                    break;
                }
            }

            // Start eliminating characters from the left, until the window is no longer valid. At
            // that point, we'll have the minimum length (valid) window that finishes just before
            // window.end.
            loop {
                if window.start == window.end {
                    break;
                }

                // detect when eliminating the next char would invalidate the window
                let ci = char_index(s[window.start]);
                if st[ci] == tt[ci] {
                    // This is the locally-minimum valid window; record it.
                    if let Some(ref mut w) = minimum_window {
                        if window.len() < w.len() {
                            *w = window.clone();
                        }
                    } else {
                        minimum_window = Some(window.clone());
                    }

                    st[ci] -= 1;
                    window.start += 1;
                    break;
                }

                st[ci] -= 1;
                window.start += 1;
            }
        }
    }
}

#[test]
fn do_test() {
    assert_eq!(
        Solution::min_window("ADOBECODEBANC".into(), "ABC".into()),
        "BANC".to_string()
    );
    assert_eq!(
        Solution::min_window("a".into(), "a".into()),
        "a".to_string()
    );
    assert_eq!(
        Solution::min_window("a".into(), "aa".into()),
        "".to_string()
    );
}
