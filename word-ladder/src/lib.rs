// Problem description: https://leetcode.com/problems/word-ladder/

#![allow(dead_code)]

struct Solution;

impl Solution {
    fn edit_distance(a: &str, b: &str) -> usize {
        a.chars().zip(b.chars()).filter(|(a, b)| a != b).count()
    }

    // TODO: begin_word might exist inside word_list
    pub fn ladder_length(begin_word: String, end_word: String, mut word_list: Vec<String>) -> i32 {
        if !word_list.contains(&end_word) {
            return 0;
        } else if begin_word == end_word {
            return 1;
        }

        let begin_index;
        let end_index = word_list.iter().position(|w| *w == end_word).unwrap();

        if let Some(index) = word_list.iter().position(|w| *w == begin_word) {
            begin_index = index;
        } else {
            word_list.push(begin_word);
            begin_index = word_list.len() - 1;
        }

        let mut edges = vec![vec![]; word_list.len()];
        for i in 0..word_list.len() {
            for j in i + 1..word_list.len() {
                if Self::edit_distance(&word_list[i], &word_list[j]) == 1 {
                    edges[i].push(j);
                    edges[j].push(i);
                }
            }
        }

        println!("{:?}", edges);

        let mut n = 1;
        let mut explored = vec![false; word_list.len()];

        let mut adjacent = edges[begin_index].clone();
        explored[begin_index] = true;

        while !adjacent.is_empty() {
            let mut next_adj: Vec<usize> = vec![];
            for edge in adjacent {
                if edge == end_index {
                    return n + 1;
                }

                for next in edges[edge].iter().copied() {
                    if explored[next] {
                        continue;
                    } else {
                        explored[next] = true;
                    }

                    next_adj.push(next);
                }
            }

            adjacent = next_adj;
            n += 1;
        }

        0
    }
}

#[test]
fn do_test() {
    let words: Vec<String> = vec![
        "hot".into(),
        "dot".into(),
        "dog".into(),
        "lot".into(),
        "log".into(),
        "cog".into(),
    ];

    assert_eq!(
        Solution::ladder_length("hit".into(), "cog".into(), words),
        5
    );

    let words: Vec<String> = vec![
        "hot".into(),
        "dot".into(),
        "dog".into(),
        "lot".into(),
        "log".into(),
    ];

    assert_eq!(
        Solution::ladder_length("hit".into(), "cog".into(), words),
        0
    );

    let words: Vec<String> = vec![
        "dot".into(),
        "dog".into(),
        "lot".into(),
        "log".into(),
        "cog".into(),
    ];

    assert_eq!(
        Solution::ladder_length("hit".into(), "cog".into(), words),
        0
    );
}
