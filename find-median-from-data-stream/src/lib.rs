#![allow(dead_code)]

#[derive(Default)]
struct MedianFinder {
    nums: Vec<i32>,
}

impl MedianFinder {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn find_index(&self, num: i32, range: std::ops::Range<usize>) -> usize {
        let mid = (range.start + range.end) / 2;

        if range.is_empty() {
            range.start
        } else if range.len() == 1 {
            if self.nums[range.start] < num {
                range.end
            } else {
                range.start
            }
        } else if num < self.nums[mid] {
            self.find_index(num, range.start..mid)
        } else if num == self.nums[mid] {
            mid
        } else {
            self.find_index(num, mid + 1..range.end)
        }
    }

    pub fn add_num(&mut self, num: i32) {
        self.nums
            .insert(self.find_index(num, 0..self.nums.len()), num)
    }

    pub fn find_median(&self) -> f64 {
        assert!(!self.nums.is_empty());
        let l = self.nums.len();
        if l % 2 == 0 {
            (self.nums[l / 2 - 1] + self.nums[l / 2]) as f64 / 2.0f64
        } else {
            self.nums[l / 2] as f64
        }
    }
}

#[test]
fn do_test() {
    let mut median_finder = MedianFinder::new();
    median_finder.add_num(1);
    assert_eq!(median_finder.find_median(), 1.0);
    median_finder.add_num(2);
    assert_eq!(median_finder.find_median(), 1.5);
    median_finder.add_num(3);
    assert_eq!(median_finder.find_median(), 2.0);

    let mut median_finder = MedianFinder::new();
    median_finder.add_num(3);
    assert_eq!(median_finder.find_median(), 3.0);
    median_finder.add_num(2);
    assert_eq!(median_finder.find_median(), 2.5);
    median_finder.add_num(1);
    assert_eq!(median_finder.find_median(), 2.0);

    let mut median_finder = MedianFinder::new();
    median_finder.add_num(1);
    assert_eq!(median_finder.find_median(), 1.0);
    median_finder.add_num(3);
    assert_eq!(median_finder.find_median(), 2.0);
    median_finder.add_num(2);
    assert_eq!(median_finder.find_median(), 2.0);
}
