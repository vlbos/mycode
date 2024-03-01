// 1271\. Hexspeak
// ===============

// A decimal number can be converted to its _Hexspeak representation_ by first converting it to an uppercase hexadecimal string,
// then replacing all occurrences of the digit `0` with the letter `O`, and the digit `1` with the letter `I`.
// Such a representation is _valid_ if and only if it consists only of the letters in the set `{"A", "B", "C", "D", "E", "F", "I", "O"}`.

// Given a string `num` representing a decimal integer `N`, return the Hexspeak representation of `N` if it is valid, otherwise return `"ERROR"`.

// **Example 1:**

// **Input:** num = "257"
// **Output:** "IOI"
// **Explanation: ** 257 is 101 in hexadecimal.

// **Example 2:**

// **Input:** num = "3"
// **Output:** "ERROR"

// **Constraints:**

// *   `1 <= N <= 10^12`
// *   There are no leading zeros in the given string.
// *   All answers must be in uppercase letters.

// ### Difficulty:

// Easy

// ### Lock:

// Prime

// ### Company:

// [Virtu Financial](https://leetcode.ca/tags/#Virtu%20Financial)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn to_hexspeak(num: String) -> String {
        let n = num.parse::<i64>().unwrap();
        let mut hex = format!("{:X}", n);
        if hex
            .bytes()
            .any(|b| b != b'0' && b != b'1' && (b < b'A' || b > b'F'))
        {
            return "ERROR".to_string();
        }
        hex = hex.replace("1", "I");
        hex = hex.replace("0", "O");
        hex
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_to_hexspeak_1() {
        assert_eq!(
            String::from("IOI"),
            Solution::to_hexspeak(String::from("257"))
        );
    }
    #[test]
    pub fn test_to_hexspeak_2() {
        assert_eq!(
            String::from("ERROR"),
            Solution::to_hexspeak(String::from("3"))
        );
    }
    #[test]
    pub fn test_to_hexspeak_3() {
        assert_eq!(String::from("F"), Solution::to_hexspeak(String::from("15")));
    }
}
