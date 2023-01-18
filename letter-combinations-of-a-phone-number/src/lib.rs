// https://leetcode.com/problems/letter-combinations-of-a-phone-number/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let digits: Vec<char> = digits.chars().collect();

        let mut combos = Vec::<String>::new();

        if digits.is_empty() {
            return combos;
        }

        // add the empty string here, so that we can progress by doing the cartesian product of
        // 'combos' with 'letters' repeatedly.
        combos.push("".into());

        for d in digits {
            let letters: &[char] = match d {
                '2' => &['a', 'b', 'c'],
                '3' => &['d', 'e', 'f'],
                '4' => &['g', 'h', 'i'],
                '5' => &['j', 'k', 'l'],
                '6' => &['m', 'n', 'o'],
                '7' => &['p', 'q', 'r', 's'],
                '8' => &['t', 'u', 'v'],
                '9' => &['w', 'x', 'y', 'z'],
                _ => panic!("invalid input digit {d}"),
            };

            combos = combos
                .iter()
                .flat_map(|combo| {
                    letters.iter().map(move |letter| {
                        let mut x = combo.clone();
                        x.push(*letter);
                        x
                    })
                })
                .collect();
        }

        combos
    }
}

#[test]
fn do_test() {
    assert_eq!(
        Solution::letter_combinations("23".into()),
        vec![
            "ad".to_string(),
            "ae".to_string(),
            "af".to_string(),
            "bd".to_string(),
            "be".to_string(),
            "bf".to_string(),
            "cd".to_string(),
            "ce".to_string(),
            "cf".to_string()
        ]
    );

    assert_eq!(
        Solution::letter_combinations("".to_string()),
        Vec::<String>::new()
    );

    assert_eq!(
        Solution::letter_combinations("2".to_string()),
        vec!["a".to_string(), "b".to_string(), "c".to_string()]
    );
}
