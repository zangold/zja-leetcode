#![allow(dead_code)]

struct Solution;

impl Solution {
    // Find the index of x (or where x would go) in 'sorted'. Needs to be the smallest possible
    // index.
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

    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let mut output = vec![0i32; nums.len()];

        // Effectively do an insertion sort from right to left on the input array. Store the sorted
        // array in 'sorted'
        let mut sorted = Vec::<i32>::new();

        let mut do_step = |index: usize, x: i32| {
            let num_less = Self::find_index(&sorted, x, 0..sorted.len());
            sorted.insert(num_less, x);
            output[index] = num_less as i32;
        };

        nums.iter()
            .enumerate()
            .rev()
            .for_each(|(index, x)| do_step(index, *x));

        output
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::count_smaller(vec![5, 2, 6, 1]), vec![2, 1, 1, 0]);
    assert_eq!(Solution::count_smaller(vec![-1]), vec![0]);
    assert_eq!(Solution::count_smaller(vec![-1, -1]), vec![0, 0]);
}
