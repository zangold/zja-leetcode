// https://leetcode.com/problems/lexicographically-smallest-equivalent-string/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();

        let mut sets = [0u8; 26];
        let mut set_mins: [u8; 26] = [b'z'; 26];

        for (i, set) in sets.iter_mut().enumerate() {
            *set = i as u8;
        }

        fn set_id(sets: &mut [u8], c: u8) -> u8 {
            if sets[c as usize] != c {
                sets[c as usize] = set_id(sets, sets[c as usize])
            }

            sets[c as usize]
        }

        for (c1, c2) in s1.iter().copied().zip(s2.iter().copied()) {
            let c1 = c1 - b'a';
            let c2 = c2 - b'a';

            let set1 = set_id(&mut sets, c1);
            let set2 = set_id(&mut sets, c2);

            sets[set2 as usize] = set1;
        }

        for i in 0..26 {
            let set = set_id(&mut sets, i as u8);
            set_mins[set as usize] = set_mins[set as usize].min(i as u8 + b'a');
        }

        let mut output = "".to_string();
        for c in base_str.chars() {
            let set = set_id(&mut sets, c as u8 - b'a');
            output.push((set_mins[set as usize]) as char);
        }

        output
    }
}

#[test]
fn do_test() {
    assert_eq!(
        Solution::smallest_equivalent_string("parker".into(), "morris".into(), "parser".into()),
        "makkek".to_string()
    );
    assert_eq!(
        Solution::smallest_equivalent_string("hello".into(), "world".into(), "hold".into()),
        "hdld".to_string()
    );
    assert_eq!(
        Solution::smallest_equivalent_string(
            "leetcode".into(),
            "programs".into(),
            "sourcecode".into()
        ),
        "aauaaaaada".to_string()
    );
}
