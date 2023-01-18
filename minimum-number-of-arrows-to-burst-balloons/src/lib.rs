// https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by_key(|p| p[0]);

        let mut remaining: &[Vec<i32>] = &points[..];
        let mut num_arrows = 0;

        while !remaining.is_empty() {
            let mut min_shot = remaining[0][0];
            let mut max_shot = remaining[0][1];

            let mut num_hit = 1;
            while num_hit < remaining.len() {
                min_shot = i32::max(min_shot, remaining[num_hit][0]);
                max_shot = i32::min(max_shot, remaining[num_hit][1]);

                if min_shot > max_shot {
                    break;
                }

                num_hit += 1;
            }

            remaining = &remaining[num_hit..];
            num_arrows += 1;
        }

        num_arrows
    }
}

fn make_test_input(points: &[(i32, i32)]) -> Vec<Vec<i32>> {
    points.iter().copied().map(|(a, b)| vec![a, b]).collect()
}

#[test]
fn do_test() {
    let points = make_test_input(&[(10, 16), (2, 8), (1, 6), (7, 12)]);
    assert_eq!(Solution::find_min_arrow_shots(points), 2);

    let points = make_test_input(&[(1, 2), (3, 4), (5, 6), (7, 8)]);
    assert_eq!(Solution::find_min_arrow_shots(points), 4);

    let points = make_test_input(&[(1, 2), (2, 3), (3, 4), (4, 5)]);
    assert_eq!(Solution::find_min_arrow_shots(points), 2);
}
