// Problem description: https://leetcode.com/problems/cheapest-flights-within-k-stops/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut costs = vec![-1; n as usize];

        // current location, cost to get there
        let mut ds = vec![(src, 0)];
        let mut num_stops = 0;

        // dst, cost
        let mut flights_from = vec![vec![]; n as usize];

        for f in flights {
            flights_from[f[0] as usize].push((f[1], f[2]));
        }

        while !ds.is_empty() && num_stops <= k {
            let mut adj = vec![];

            for (city, cost) in ds {
                for (next_city, fcost) in flights_from[city as usize].iter() {
                    if costs[*next_city as usize] > cost + fcost || costs[*next_city as usize] == -1
                    {
                        costs[*next_city as usize] = cost + fcost;
                        adj.push((*next_city, cost + fcost));
                    }
                }
            }

            ds = adj;
            num_stops += 1
        }

        costs[dst as usize]
    }
}

#[test]
fn do_test() {
    let flights = vec![
        vec![0, 1, 100],
        vec![1, 2, 100],
        vec![2, 0, 100],
        vec![1, 3, 600],
        vec![2, 3, 200],
    ];

    assert_eq!(Solution::find_cheapest_price(4, flights, 0, 3, 1), 700);

    let flights = vec![
        vec![0, 1, 100],
        vec![1, 2, 100],
        vec![2, 0, 100],
        vec![1, 3, 600],
        vec![2, 3, 200],
    ];

    assert_eq!(Solution::find_cheapest_price(4, flights, 0, 3, 2), 400);

    let flights = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];

    assert_eq!(Solution::find_cheapest_price(3, flights, 0, 2, 1), 200);

    let flights = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];

    assert_eq!(Solution::find_cheapest_price(3, flights, 0, 2, 0), 500);
}
