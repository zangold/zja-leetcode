#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let strs = strs
            .into_iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<_>>();

        (0..strs[0].len())
            .filter(|i| !(1..strs.len()).all(|j| strs[j - 1][*i] <= strs[j][*i]))
            .count() as i32
    }
}

#[test]
fn do_test() {
    assert_eq!(
        Solution::min_deletion_size(vec!["cba".into(), "daf".into(), "ghi".into()]),
        1
    );

    assert_eq!(Solution::min_deletion_size(vec!["a".into(), "b".into()]), 0);

    assert_eq!(
        Solution::min_deletion_size(vec!["zyx".into(), "wvu".into(), "tsr".into()]),
        3
    );

    assert_eq!(
        Solution::min_deletion_size(vec!["aaa".into(), "aaa".into(), "aaa".into()]),
        0
    );
}
