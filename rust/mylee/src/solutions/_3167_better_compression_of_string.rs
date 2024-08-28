// # [3167. Better Compression of String 🔒](https://leetcode.com/problems/better-compression-of-string)

// ## Description

// You are given a string compressed representing a compressed version of a string.
// The format is a character followed by its frequency.
//  For example, "a3b1a1c2" is a compressed version of the string "aaabacc".

// We seek a better compression with the following conditions:

// 	Each character should appear only once in the compressed version.
// 	The characters should be in alphabetical order.

// Return the better compression of compressed.

// Note: In the better version of compression, the order of letters may change, which is acceptable.

//
// Example 1:

// Input: compressed = "a3c9b2c1"

// Output: "a3b2c10"

// Explanation:

// Characters "a" and "b" appear only once in the input, but "c" appears twice, once with a size of 9 and once with a size of 1.

// Hence, in the resulting string, it should have a size of 10.

// Example 2:

// Input: compressed = "c2b3a1"

// Output: "a1b3c2"

// Example 3:

// Input: compressed = "a2b4c1"

// Output: "a2b4c1"

//
// Constraints:

// 	1 <= compressed.length <= 6 * 104
// 	compressed consists only of lowercase English letters and digits.
// 	compressed is a valid compression, i.e., each character is followed by its frequency.
// 	Frequencies are in the range [1, 104] and have no leading zeroes.

//     string better_compression(string compressed) {

impl Solution {
    pub fn better_compression(compressed: String) -> String {
        let mut m = std::collections::BTreeMap::new();
        let mut cnt = 0;
        let mut letter = ' ';
        for c in compressed.chars() {
            if c.is_ascii_digit() {
                cnt = cnt * 10 + c.to_digit(10).unwrap();
                continue;
            }
            if cnt > 0 {
                *m.entry(letter).or_insert(0) += cnt;
            }
            letter = c;
            cnt = 0;
        }
        if cnt > 0 {
            *m.entry(letter).or_insert(0) += cnt;
        }
        m.into_iter()
            .map(|(k, v)| format!("{k}{v}"))
            .collect::<Vec<_>>()
            .concat()
    }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_better_compression_1() {
        assert_eq!(
            Solution::better_compression(String::from("a3c9b2c1")),
            "a3b2c10"
        );
    }

    #[test]
    pub fn test_better_compression_2() {
        assert_eq!(
            Solution::better_compression(String::from("c2b3a1")),
            "a1b3c2"
        );
    }

    #[test]
    pub fn test_better_compression_3() {
        assert_eq!(
            Solution::better_compression(String::from("a2b4c1")),
            "a2b4c1"
        );
    }
}
