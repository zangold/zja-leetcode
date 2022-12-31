#![allow(dead_code)]

struct Solution;

impl Solution {
    pub const fn divide(mut dividend: i32, mut divisor: i32) -> i32 {
        let mut quotient: i32 = 0;

        // use this to deal with edge cases where dividend == i32::MIN, since
        // i32::MIN == -i32::MAX - 1
        let mut quotient_fudge_factor = 0;

        // first, deal with the edge cases where dividend or divisor could be i32::MIN, or we end
        // up with division overflow.
        if dividend == i32::MIN && divisor == i32::MIN {
            return 1;
        } else if divisor == i32::MIN {
            return 0;
        } else if dividend == i32::MIN {
            if divisor == -1 {
                return i32::MAX;
            } else if divisor > 0 {
                dividend += divisor;
                quotient_fudge_factor += 1;
            } else {
                dividend -= divisor;
                quotient_fudge_factor += 1;
            }
        }

        // Now, make sure both dividend and divisor are positive.
        let dividend_negative = dividend < 0;
        dividend = dividend.abs();

        let divisor_negative = divisor < 0;
        divisor = divisor.abs();

        let quotient_negative = dividend_negative != divisor_negative;

        // Now do some bit-shifting shenanigans.
        let mut divisor_shift: i32 = 0;
        while (divisor << divisor_shift) & 0x4000_0000 == 0 {
            divisor_shift += 1;
        }

        while divisor_shift >= 0 {
            if dividend >= (divisor << divisor_shift) {
                dividend -= divisor << divisor_shift;
                quotient += 1 << divisor_shift;
            }

            divisor_shift -= 1;
        }

        if quotient_negative {
            -quotient - quotient_fudge_factor
        } else {
            quotient + quotient_fudge_factor
        }
    }
}

fn test_helper(dividend: i32, divisor: i32) -> bool {
    let q1 = dividend / divisor;
    let q2 = Solution::divide(dividend, divisor);

    if q1 != q2 {
        panic!("incorrect result for {dividend} / {divisor}: {q2}, should be {q1}");
    }

    q1 == q2
}

#[test]
fn do_test() {
    test_helper(10, 3);
    test_helper(7, -3);

    test_helper(i32::MIN, i32::MIN);
    test_helper(i32::MAX, i32::MIN);
    test_helper(i32::MIN, i32::MAX);
    test_helper(i32::MAX, i32::MAX);

    test_helper(i32::MIN, 1);

    assert_eq!(Solution::divide(i32::MIN, -1), i32::MAX);
    test_helper(i32::MAX, -1);
    test_helper(i32::MAX, 1);

    for i in -100..100 {
        for j in -100..100 {
            if j == 0 {
                continue;
            }

            test_helper(i, j);
        }
    }
}
