// https://leetcode.com/problems/median-of-two-sorted-arrays/

#![allow(dead_code)]

struct Solution;

impl Solution {
    // Solver for base case which just merges the two arrays and selects the middle element. Only
    // call this with at most two elements in 'short', to keep runtime low.
    fn find_median_merge(short: &[i32], rest: &[i32]) -> f64 {
        assert!(short.len() <= 2);

        // For even-length arrays, take the middle two elements; for odd-length, take the middle
        // three elements. This is the same as removing (len / 2 - 1) elements from each side.
        let remove_elements = if rest.len() <= 5 {
            0
        } else {
            rest.len() / 2 - 2
        };
        let short_rest = &rest[remove_elements..rest.len() - remove_elements];

        // Merge rest and short_rest, and then figure out the median.
        let mut all_elements = short
            .iter()
            .chain(short_rest)
            .copied()
            .collect::<Vec<i32>>();

        all_elements.sort_unstable();

        println!("all_elements: {:?}", all_elements);

        if all_elements.len() % 2 == 0 {
            // Average of the middle two elements
            let a = all_elements[all_elements.len() / 2 - 1] as f64;
            let b = all_elements[all_elements.len() / 2] as f64;

            (a + b) / 2.0_f64
        } else {
            // Just the middle element
            all_elements[all_elements.len() / 2] as f64
        }
    }

    // Checks base cases. If a base case was found, returns Some(f64), otherwise None.
    fn find_median_base(nums1: &[i32], nums2: &[i32]) -> Option<f64> {
        match (nums1.len(), nums2.len()) {
            (x, _) if x <= 2 => Some(Self::find_median_merge(nums1, nums2)),
            (_, y) if y <= 2 => Some(Self::find_median_merge(nums2, nums1)),
            (_, _) => None,
        }
    }

    fn find_median_recursive(nums1: &[i32], nums2: &[i32]) -> f64 {
        if let Some(median) = Self::find_median_base(nums1, nums2) {
            return median;
        }

        // First, select representatives from (roughly) the middle of each of the two ranges.
        let r1 = nums1.len() / 2;
        let r2 = nums2.len() / 2;

        // Always remove (len - 1) / 2 elements.
        let remove_elements = std::cmp::min((nums1.len() - 1) / 2, (nums2.len() - 1) / 2);

        assert_ne!(remove_elements, 0);

        // The median must be somewhere between r1 and r2 in the sorted (merged) array. We can
        // remove any of the elements outside of that range (except the closest one), but we have
        // to remove an equal number of elements from each side so that the median of the new
        // arrays is still the same.
        if nums1[r1] < nums2[r2] {
            // remove from the bottom of nums1 and from the top of nums2
            Self::find_median_recursive(
                &nums1[remove_elements..],
                &nums2[..nums2.len() - remove_elements],
            )
        } else {
            // remove from the bottom of nums2 and from the top of nums1
            Self::find_median_recursive(
                &nums2[remove_elements..],
                &nums1[..nums1.len() - remove_elements],
            )
        }
    }

    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        Self::find_median_recursive(&nums1[..], &nums2[..])
    }
}

fn naive_solution(nums1: &[i32], nums2: &[i32]) -> f64 {
    let mut all_elements = nums1
        .iter()
        .chain(nums2.iter())
        .copied()
        .collect::<Vec<i32>>();

    all_elements.sort_unstable();

    if all_elements.len() % 2 == 0 {
        // Average of the middle two elements
        let a = all_elements[all_elements.len() / 2 - 1] as f64;
        let b = all_elements[all_elements.len() / 2] as f64;

        (a + b) / 2.0_f64
    } else {
        // Just the middle element
        all_elements[all_elements.len() / 2] as f64
    }
}

#[test]
fn do_test() {
    use rand::Rng;

    assert_eq!(
        2.0_f64,
        Solution::find_median_sorted_arrays(vec![1, 3], vec![2])
    );
    assert_eq!(
        2.5_f64,
        Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4])
    );

    // Try generating some arrays, each of random lengths between 0 and 100, with random contents,
    // and see if the fast solution matches the naive solution.
    for m in 0..20 {
        for n in 0..20 {
            if m == 0 && n == 0 {
                continue;
            }

            let mut v1 = (0..m)
                .into_iter()
                .map(|_| rand::thread_rng().gen_range(0..100))
                .collect::<Vec<i32>>();
            let mut v2 = (0..n)
                .into_iter()
                .map(|_| rand::thread_rng().gen_range(0..100))
                .collect::<Vec<i32>>();

            v1.sort_unstable();
            v2.sort_unstable();

            println!("v1 = {:?}", v1);
            println!("v2 = {:?}", v2);

            let s1 = naive_solution(&v1, &v2);
            let s2 = Solution::find_median_sorted_arrays(v1, v2);

            assert_eq!(s1, s2);
            println!("test case passed");
        }
    }
}
