#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut used = [0i32; 32];

        if s.len() != t.len() {
            return false;
        }

        s.chars().for_each(|c| used[c as usize & 0x1f] += 1);

        for c in t.chars() {
            let ci = c as usize & 0x1f;
            if used[ci] == 0 {
                return false;
            }

            used[ci] -= 1;
        }

        true
    }
}

#[test]
fn do_test() {
    assert!(Solution::is_anagram("anagram".into(), "nagaram".into()));
    assert!(!Solution::is_anagram("rat".into(), "car".into()));
}
