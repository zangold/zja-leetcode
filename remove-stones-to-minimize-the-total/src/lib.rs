#![allow(dead_code)]

use std::collections::BTreeMap;

struct BTreeMultiSet<T> {
    map: BTreeMap<T, usize>,
}

impl<T: Ord + Copy> BTreeMultiSet<T> {
    pub fn new() -> BTreeMultiSet<T> {
        BTreeMultiSet::<T> {
            map: BTreeMap::<T, usize>::new(),
        }
    }

    pub fn insert(&mut self, x: &T) {
        match self.map.get_mut(x) {
            Some(count) => {
                *count += 1;
            }
            None => {
                self.map.insert(*x, 1);
            }
        }
    }

    pub fn remove(&mut self, x: &T) {
        match self.map.get_mut(x) {
            Some(count) => {
                *count -= 1;

                if *count == 0 {
                    self.map.remove(x);
                }
            }
            None => {
                panic!("shouldn't be able to get here");
            }
        }
    }

    pub fn get_highest(&self) -> Option<&T> {
        if let Some((set_element, _)) = self.map.iter().next_back() {
            Some(set_element)
        } else {
            None
        }
    }
}

struct Solution;

impl Solution {
    pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        let mut bt = BTreeMultiSet::<i32>::new();
        let mut total = 0;

        piles.iter().for_each(|p| {
            bt.insert(p);
            total += p;
        });

        for _ in 0..k {
            let pile: i32 = *bt.get_highest().unwrap();
            bt.remove(&pile);
            let remove = pile / 2;
            total -= remove;
            bt.insert(&(pile - remove));
        }

        total
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::min_stone_sum(vec![5, 4, 9], 2), 12);
    assert_eq!(Solution::min_stone_sum(vec![4, 3, 6, 7], 3), 12);
}
