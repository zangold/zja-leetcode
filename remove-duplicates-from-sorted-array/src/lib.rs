#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut cursor = 0;

        for i in 1..nums.len() {
            if nums[i] != nums[cursor] {
                cursor += 1;
                nums[cursor] = nums[i];
            }
        }

        (cursor + 1) as i32
    }
}

fn naive_solution(nums: &[i32]) -> Vec<i32> {
    let mut output = vec![nums[0]];

    for i in 1..nums.len() {
        if nums[i] != nums[i - 1] {
            output.push(nums[i]);
        }
    }

    output
}

fn do_test(nums: &[i32]) {
    let naive = naive_solution(nums);

    let mut v: Vec<i32> = nums.to_vec();
    let k = Solution::remove_duplicates(&mut v) as usize;

    // Solution shouldn't change the length of the vector it was given.
    assert_eq!(v.len(), nums.len());

    assert_eq!(naive, &v[0..k]);
}

#[test]
fn main() {
    do_test(&[1, 1, 2]);
    do_test(&[0, 0, 1, 1, 1, 2, 2, 3, 3, 4]);
}
