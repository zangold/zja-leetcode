#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut n = [0usize; 3];

        for num in nums.iter() {
            n[*num as usize] += 1;
        }

        nums.iter_mut().skip(0).take(n[0]).for_each(|num| *num = 0);
        nums.iter_mut()
            .skip(n[0])
            .take(n[1])
            .for_each(|num| *num = 1);
        nums.iter_mut()
            .skip(n[0] + n[1])
            .take(n[2])
            .for_each(|num| *num = 2);
    }
}

#[test]
fn do_test() {
    let mut v = vec![2, 0, 2, 1, 1, 0];
    Solution::sort_colors(&mut v);
    assert_eq!(v, vec![0, 0, 1, 1, 2, 2]);

    let mut v = vec![2, 0, 1];
    Solution::sort_colors(&mut v);
    assert_eq!(v, vec![0, 1, 2]);
}
