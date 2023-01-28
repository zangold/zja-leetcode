// Problem description: https://leetcode.com/problems/data-stream-as-disjoint-intervals/

#![allow(dead_code)]

struct SummaryRanges {
    in_stream: [bool; 10001],
}

impl SummaryRanges {
    fn new() -> Self {
        Self {
            in_stream: [false; 10001],
        }
    }

    fn add_num(&mut self, value: i32) {
        let value = value as usize;
        if !self.in_stream[value] {
            self.in_stream[value] = true;
        }
    }

    fn get_intervals(&mut self) -> Vec<Vec<i32>> {
        let mut intervals = vec![];

        let mut i = 0;
        loop {
            while !self.in_stream[i] {
                i += 1;

                if i == 10001 {
                    return intervals;
                }
            }

            let mut new_interval = vec![i as i32, 0];
            while self.in_stream[i] {
                i += 1;

                if i == 10001 {
                    new_interval[1] = 10000;
                    intervals.push(new_interval);
                    return intervals;
                }
            }

            new_interval[1] = (i - 1) as i32;
            intervals.push(new_interval);
        }
    }
}

#[test]
fn do_test() {
    let mut sr = SummaryRanges::new();

    sr.add_num(1);
    assert_eq!(sr.get_intervals(), vec![vec![1, 1]]);
    sr.add_num(3);
    assert_eq!(sr.get_intervals(), vec![vec![1, 1], vec![3, 3]]);
    sr.add_num(7);
    assert_eq!(sr.get_intervals(), vec![vec![1, 1], vec![3, 3], vec![7, 7]]);
    sr.add_num(2);
    assert_eq!(sr.get_intervals(), vec![vec![1, 3], vec![7, 7]]);
    sr.add_num(6);
    assert_eq!(sr.get_intervals(), vec![vec![1, 3], vec![6, 7]]);
}
