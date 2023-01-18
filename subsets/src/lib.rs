#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        (0..(1 << nums.len()))
            .into_iter()
            .map(|x| {
                nums.iter()
                    .copied()
                    .enumerate()
                    .filter_map(|(index, num)| {
                        if (x & (1 << index)) != 0 {
                            Some(num)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

#[test]
fn do_test() {
    let nums = vec![1, 2, 3];
    let expected = vec![
        vec![],
        vec![1],
        vec![2],
        vec![1, 2],
        vec![3],
        vec![1, 3],
        vec![2, 3],
        vec![1, 2, 3],
    ];
    assert_eq!(Solution::subsets(nums), expected);

    let nums = vec![0];
    let expected = vec![vec![], vec![0]];
    assert_eq!(Solution::subsets(nums), expected);
}
