// Problem description: https://leetcode.com/problems/restore-ip-addresses/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn helper(s: &[char], ip: &mut Vec<u8>, all_ips: &mut Vec<String>) {
        if ip.len() == 4 {
            if s.is_empty() {
                all_ips.push(format!("{}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3]))
            }
            return;
        }

        let c2i = |c: char| c as usize - b'0' as usize;

        // Can take either 1, 2, or 3 chars.
        let mut byte = 0usize;
        for i in 0..3.min(s.len()) {
            byte = byte * 10 + c2i(s[i]);
            if byte < 256 {
                ip.push(byte as u8);
                Self::helper(&s[i + 1..], ip, all_ips);
                ip.pop();
            }

            // can't have 032, for example
            if i == 0 && byte == 0 {
                break;
            }
        }
    }

    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let s = s.chars().collect::<Vec<char>>();
        let mut ips = vec![];

        Self::helper(&s, &mut vec![], &mut ips);

        ips
    }
}

#[test]
fn do_test() {
    let input = "25525511135".to_string();
    let mut output = Solution::restore_ip_addresses(input);

    let mut expected = vec!["255.255.11.135".to_string(), "255.255.111.35".to_string()];

    output.sort_unstable();
    expected.sort_unstable();
    assert_eq!(output, expected);

    let input = "0000".to_string();
    let output = Solution::restore_ip_addresses(input);
    let expected = vec!["0.0.0.0".to_string()];
    assert_eq!(output, expected);

    let input = "101023".to_string();
    let mut output = Solution::restore_ip_addresses(input);

    let mut expected = vec![
        "1.0.10.23".to_string(),
        "1.0.102.3".to_string(),
        "10.1.0.23".to_string(),
        "10.10.2.3".to_string(),
        "101.0.2.3".to_string(),
    ];

    output.sort_unstable();
    expected.sort_unstable();
    assert_eq!(output, expected);
}
