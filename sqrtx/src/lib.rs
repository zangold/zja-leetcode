#![allow(dead_code)]

struct Solution;

impl Solution {
    /// Try a binary search on an inclusive range of possible values to find the square root of x.
    fn find_sqrt(x: u64, range: std::ops::RangeInclusive<u64>) -> i32 {
        assert!(!range.is_empty());

        if *range.start() == *range.end() {
            assert!(range.start() * range.start() <= x);
            return *range.start() as i32;
        }

        let mid = (*range.end() - *range.start()) / 2 + *range.start();

        if mid * mid > x {
            Self::find_sqrt(x, *range.start()..=mid - 1)
        } else if mid * mid <= x && (mid + 1) * (mid + 1) > x {
            mid as i32
        } else {
            Self::find_sqrt(x, mid + 1..=*range.end())
        }
    }

    pub fn my_sqrt(x: i32) -> i32 {
        Self::find_sqrt(x as u64, 0..=x as u64)
    }
}

#[test]
fn do_test() {
    for i in 0i32..1000i32 {
        let sqrt = Solution::my_sqrt(i);

        assert!(sqrt * sqrt <= i && (sqrt + 1) * (sqrt + 1) > i);
    }

    assert_eq!(Solution::my_sqrt(4), 2);
    assert_eq!(Solution::my_sqrt(8), 2);
}
