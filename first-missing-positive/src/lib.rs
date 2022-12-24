#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            // swap nums[i] with nums[nums[i]] if nums[i] is a valid index.
            while nums[i] != (i + 1) as i32 && (1..=nums.len() as i32).contains(&nums[i]) {
                // valid because nums[i] is greater than 0
                let n = (nums[i] - 1) as usize;

                if nums[i] == nums[n] {
                    break;
                }

                nums.swap(n, i);
            }
        }

        for (i, num) in nums.iter().enumerate() {
            if *num != (i + 1) as i32 {
                return (i + 1) as i32;
            }
        }

        nums[nums.len() - 1] + 1
    }
}

fn naive_solution(nums: &[i32]) -> i32 {
    let mut v = nums.to_vec();

    v.sort_unstable();

    for i in 1..nums.len() + 2 {
        let x = i as i32;

        if !nums.contains(&x) {
            return i as i32;
        }
    }

    panic!("shouldn't be able to get here");
}

fn test_solution(nums: &[i32]) -> bool {
    let a = naive_solution(nums);
    let b = Solution::first_missing_positive(nums.to_vec());
    println!("{} vs {}", a, b);
    a == b
}

#[test]
fn do_tests() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
    assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
    assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);

    assert!(test_solution(&[0, 1, 2, 3, 4]));
    assert!(test_solution(&[-1, 0, 1, 2, 3, 4]));
    assert!(test_solution(&[-2, 0, 2, 2, 4, 4]));
    assert!(test_solution(&[1, 2, 3, 4, 5, 6]));
    assert!(test_solution(&[2, 3, 4, 5, 6]));
}
