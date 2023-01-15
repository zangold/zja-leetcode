#![allow(dead_code)]

use std::cmp::Ordering::*;

struct Solution;

impl Solution {
    fn find_rotation(nums: &[i32]) -> usize {
        if nums.len() < 2 || nums[nums.len() - 1] > nums[0] {
            return 0;
        }

        // there must be some i in 0..n-1 for which nums[i] > nums[i+1]

        let mut range = 0..nums.len();
        loop {
            let mid = (range.start + range.end) / 2;

            if nums[mid] > nums[(mid + 1) % nums.len()] {
                return nums.len() - 1 - mid;
            }

            if range.is_empty() {
                panic!("ran out of items to search");
            }

            if nums[mid] <= i32::min(nums[range.start], nums[range.end - 1]) {
                range = range.start..mid;
            } else {
                range = mid + 1..range.end;
            }
        }
    }

    pub fn rotated_search(nums: &[i32], k: i32, target: i32, range: std::ops::Range<i32>) -> i32 {
        if range.is_empty() {
            return -1;
        }

        let unrotate = |index| (index - k + nums.len() as i32) as usize % nums.len();

        let mid = (range.start + range.end) / 2;

        match nums[unrotate(mid)].cmp(&target) {
            Greater => Self::rotated_search(nums, k, target, range.start..mid),
            Less => Self::rotated_search(nums, k, target, mid + 1..range.end),
            Equal => unrotate(mid) as i32,
        }
    }

    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let k = Self::find_rotation(&nums);

        Self::rotated_search(&nums, k as i32, target, 0..nums.len() as i32)
    }
}

fn do_rotate(nums: &mut [i32], k: usize) {
    nums.clone_from_slice(
        &(0..nums.len())
            .into_iter()
            .map(|i| nums[(i + k) % nums.len()])
            .collect::<Vec<i32>>(),
    );
}

#[test]
fn do_test() {
    for len in 2..10usize {
        for i in 0..len {
            let mut nums = (0..len as i32).into_iter().collect::<Vec<i32>>();
            do_rotate(&mut nums, i);
            assert_eq!(Solution::find_rotation(&nums), i);
        }
    }

    let v = vec![4, 5, 6, 7, 0, 1, 2];
    for i in 0..7 {
        assert_eq!(Solution::search(v.clone(), v[i]), i as i32);
    }

    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    assert_eq!(Solution::search(vec![1], 0), -1);
}
