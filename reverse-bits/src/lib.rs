#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut x_bit: u32 = 1;
        let mut output: u32 = 0;
        let mut output_bit: u32 = 1 << 31;

        for _ in 0..32 {
            if x_bit & x != 0 {
                output |= output_bit;
            }

            output_bit >>= 1;
            x_bit <<= 1;
        }

        output
    }
}

#[test]
fn do_test() {
    assert_eq!(
        Solution::reverse_bits(0b00000010100101000001111010011100),
        0b00111001011110000010100101000000
    );

    assert_eq!(
        Solution::reverse_bits(0b11111111111111111111111111111101),
        0b10111111111111111111111111111111
    );
}
