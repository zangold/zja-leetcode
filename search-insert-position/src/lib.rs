// https://leetcode.com/problems/search-insert-position/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn find_index(sorted: &[i32], x: i32, range: std::ops::Range<usize>) -> usize {
        if range.is_empty() {
            return 0;
        } else if range.len() == 1 {
            if x <= sorted[range.start] {
                return range.start;
            } else {
                return range.end;
            }
        }

        let mid = (range.start + range.end) / 2;

        if x <= sorted[mid] {
            Self::find_index(sorted, x, range.start..mid)
        } else {
            Self::find_index(sorted, x, mid..range.end)
        }
    }

    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        Self::find_index(&nums, target, 0..nums.len()) as i32
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);

    assert_eq!(Solution::search_insert(vec![], 7), 0);
    assert_eq!(Solution::search_insert(vec![4], 7), 1);
    assert_eq!(Solution::search_insert(vec![4], 2), 0);

    assert_eq!(Solution::search_insert(vec![2, 4], 1), 0);
    assert_eq!(Solution::search_insert(vec![2, 4], 2), 0);
    assert_eq!(Solution::search_insert(vec![2, 4], 3), 1);
    assert_eq!(Solution::search_insert(vec![2, 4], 4), 1);
    assert_eq!(Solution::search_insert(vec![2, 4], 5), 2);

    assert_eq!(Solution::search_insert(vec![1, 3, 5], 0), 0);
    assert_eq!(Solution::search_insert(vec![1, 3, 5], 1), 0);
    assert_eq!(Solution::search_insert(vec![1, 3, 5], 2), 1);
    assert_eq!(Solution::search_insert(vec![1, 3, 5], 3), 1);
    assert_eq!(Solution::search_insert(vec![1, 3, 5], 4), 2);
    assert_eq!(Solution::search_insert(vec![1, 3, 5], 5), 2);
    assert_eq!(Solution::search_insert(vec![1, 3, 5], 6), 3);
}
