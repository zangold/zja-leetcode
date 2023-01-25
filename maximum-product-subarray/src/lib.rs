// Problem description: https://leetcode.com/problems/maximum-product-subarray/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_product(mut nums: Vec<i32>) -> i32 {
        let mut start = 0;
        let mut max_product = i64::MIN;

        let mut range_product = 1;

        nums.push(0);

        // non-inclusive end
        for end in 0..nums.len() {
            if nums[end] == 0 {
                if end != nums.len() - 1 {
                    max_product = max_product.max(0);
                }

                if start != end {
                    if range_product >= 0 {
                        max_product = max_product.max(range_product);
                    } else {
                        // Either we can back off to the right, or to the left
                        let mut i = 0;
                        let mut right_range_product = range_product;
                        while right_range_product < 0 && i < end - start - 1 {
                            right_range_product /= nums[start + i] as i64;
                            i += 1;
                        }

                        let mut i = end - 1;
                        let mut left_range_product = range_product;
                        while left_range_product < 0 && i > start {
                            left_range_product /= nums[i] as i64;
                            i -= 1;
                        }

                        max_product = max_product.max(right_range_product.max(left_range_product));
                    }
                }

                start = end + 1;
                range_product = 1;
            } else {
                range_product *= nums[end] as i64;
            }
        }

        max_product as i32
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::max_product(vec![2, 3, -2, 4]), 6);
    assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);
    assert_eq!(Solution::max_product(vec![-2, 0, 1]), 1);

    assert_eq!(Solution::max_product(vec![-2, -2, 0, 3, -3]), 4);
    assert_eq!(Solution::max_product(vec![0, -2, -2, 0, 3, -3, 0]), 4);
    assert_eq!(Solution::max_product(vec![-1]), -1);

    assert_eq!(Solution::max_product(vec![1, 2, 3]), 6);
    assert_eq!(Solution::max_product(vec![1, -2, 3]), 3);
    assert_eq!(Solution::max_product(vec![3, -2, 1]), 3);
    assert_eq!(Solution::max_product(vec![-3, 2, -1]), 6);
    assert_eq!(Solution::max_product(vec![3, 2, -1]), 6);
    assert_eq!(Solution::max_product(vec![-3, 2, 1]), 2);
}
