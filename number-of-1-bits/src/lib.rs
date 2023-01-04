#![allow(dead_code)]

struct Solution;

// Generate bit counts for 8-bit numbers offline
const fn bit_count_8(n: u8) -> i32 {
    let mut bit_count = 0;
    let mut i = 0;
    while i <= 7 {
        if (1 << i) & n != 0 {
            bit_count += 1;
        }
        i += 1;
    }
    bit_count
}

const fn gen_bit_counts() -> [i32; 256] {
    let mut bit_counts = [0; 256];
    let mut i = 0;

    while i < 256 {
        bit_counts[i] = bit_count_8(i as u8);
        i += 1;
    }

    bit_counts
}

const BIT_COUNTS_8: [i32; 256] = gen_bit_counts();

impl Solution {
    pub const fn hamming_weight(n: u32) -> i32 {
        // The obviously-fastest solution here is to just call i32::count_ones, but that seems like
        // cheating.

        let n = n as usize;

        BIT_COUNTS_8[n & 0xFF]
            + BIT_COUNTS_8[(n & 0xFF00) >> 8]
            + BIT_COUNTS_8[(n & 0xFF0000) >> 16]
            + BIT_COUNTS_8[(n & 0xFF000000) >> 24]
    }
}

#[test]
fn do_test() {
    for i in 0..70000 {
        assert_eq!(Solution::hamming_weight(i), i.count_ones() as i32);
    }
}
