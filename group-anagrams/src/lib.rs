#![allow(dead_code)]

struct Solution;

use std::collections::hash_map::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hs = HashMap::<[i32; 26], Vec<String>>::new();

        let char_index = |c: char| c as usize - b'a' as usize;

        for s in strs.into_iter() {
            let mut ch_uses = [0i32; 26];

            for c in s.chars() {
                ch_uses[char_index(c)] += 1;
            }

            if let Some(v) = hs.get_mut(&ch_uses) {
                v.push(s);
            } else {
                hs.insert(ch_uses, vec![s]);
            }
        }

        hs.into_iter().map(|(_, ss)| ss).collect()
    }
}

#[test]
fn do_test() {
    let input = vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ];

    let mut expected = vec![
        vec!["bat".to_string()],
        vec!["tan".to_string(), "nat".to_string()],
        vec!["eat".to_string(), "tea".to_string(), "ate".to_string()],
    ];

    let mut output = Solution::group_anagrams(input);

    output.sort_unstable();
    expected.sort_unstable();
    assert_eq!(output, expected);

    let input = vec!["".to_string()];

    let expected = vec![vec!["".to_string()]];

    assert_eq!(Solution::group_anagrams(input), expected);

    let input = vec!["a".to_string()];

    let expected = vec![vec!["a".to_string()]];

    assert_eq!(Solution::group_anagrams(input), expected);
}
