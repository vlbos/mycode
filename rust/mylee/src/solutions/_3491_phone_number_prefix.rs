// ## [3491\. Phone Number Prefix 🔒](https://leetcode.com/problems/phone-number-prefix)

// [![](https://img.shields.io/badge/Difficulty-Easy-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Easy-4051B5?style=flat-square)

// ## Description

// You are given a string array `numbers` that represents phone numbers. Return `true` if no phone number is a prefix of any other phone number; otherwise, return `false`.

// **Example 1:**

// **Input:** numbers = \["1","2","4","3"\]

// **Output:** true

// **Explanation:**

// No number is a prefix of another number, so the output is `true`.

// **Example 2:**

// **Input:** numbers = \["001","007","15","00153"\]

// **Output:** false

// **Explanation:**

// The string `"001"` is a prefix of the string `"00153"`. Thus, the output is `false`.

// **Constraints:**

// +   `2 <= numbers.length <= 50`
// +   `1 <= numbers[i].length <= 50`
// +   All numbers contain only digits `'0'` to `'9'`.

//  bool phone_prefix(vector<string>& numbers) {

#[allow(dead_code)]
pub struct Solution;
impl Solution {
    pub fn phone_prefix(numbers: Vec<String>) -> bool {
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_vec_s;
    #[test]
    pub fn test_phone_prefix_1() {
        assert!(Solution::phone_prefix(lc_vec_s!["1", "2", "4", "3"]));
    }
    #[test]
    pub fn test_phone_prefix_2() {
        assert!(!Solution::phone_prefix(lc_vec_s![
            "001", "007", "15", "00153"
        ]));
    }
}
