struct Solution;

impl Solution {
    const fn fib(n: usize) -> i32 {
        let (mut a, mut b) = (1, 1);

        if n == 0 {
            return a;
        }

        let mut i = 1;
        while i < n {
            (a, b) = (b, a + b);
            i += 1;
        }

        b
    }

    const ARRAY_LEN: usize = 46;
    const fn init_solution_table() -> [i32; Solution::ARRAY_LEN] {
        let mut table = [0; Solution::ARRAY_LEN];

        let mut i = 0;
        while i < table.len() {
            table[i] = Solution::fib(i);
            i += 1;
        }

        table
    }

    const SOLUTIONS: [i32; Solution::ARRAY_LEN] = Solution::init_solution_table();

    pub fn climb_stairs(n: i32) -> i32 {
        Solution::SOLUTIONS[n as usize]
    }
}

#[test]
fn do_test() {
    assert_eq!(Solution::climb_stairs(0), 1);
    assert_eq!(Solution::climb_stairs(1), 1);
    assert_eq!(Solution::climb_stairs(2), 2);
    assert_eq!(Solution::climb_stairs(3), 3);
    assert_eq!(Solution::climb_stairs(4), 5);
}
