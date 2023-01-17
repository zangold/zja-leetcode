#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();

        let mut num_post_0s = vec![0usize; s.len()];
        let mut num_pre_1s = vec![0usize; s.len()];

        for i in 1..s.len() {
            if s[i - 1] == '1' {
                num_pre_1s[i] = num_pre_1s[i - 1] + 1;
            } else {
                num_pre_1s[i] = num_pre_1s[i - 1];
            }
        }

        for i in (0..s.len() - 1).rev() {
            if s[i + 1] == '0' {
                num_post_0s[i] = num_post_0s[i + 1] + 1;
            } else {
                num_post_0s[i] = num_post_0s[i + 1];
            }
        }

        let mut best = s.len();
        for i in 0..s.len() {
            best = best.min(num_pre_1s[i] + num_post_0s[i]);
        }

        best as i32
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::min_flips_mono_incr("00110".into()), 1);
    assert_eq!(Solution::min_flips_mono_incr("010110".into()), 2);
    assert_eq!(Solution::min_flips_mono_incr("00011000".into()), 2);

    assert_eq!(Solution::min_flips_mono_incr("0".into()), 0);
    assert_eq!(Solution::min_flips_mono_incr("1".into()), 0);
    assert_eq!(Solution::min_flips_mono_incr("00".into()), 0);
    assert_eq!(Solution::min_flips_mono_incr("01".into()), 0);
    assert_eq!(Solution::min_flips_mono_incr("10".into()), 1);
    assert_eq!(Solution::min_flips_mono_incr("11".into()), 0);
}
