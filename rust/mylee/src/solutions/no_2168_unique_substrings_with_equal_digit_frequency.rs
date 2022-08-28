// [2168\. Unique Substrings With Equal Digit Frequency (Medium)](https://leetcode.com/problems/unique-substrings-with-equal-digit-frequency/)[](https://leetcode.ca/2022-03-01-2168-Unique-Substrings-With-Equal-Digit-Frequency/#2168-unique-substrings-with-equal-digit-frequency-medium)
// =========================================================================================================================================================================================================================================================================================

// Given a digit string `s`, return _the number of **unique substrings** of_ `s` _where every digit appears the same number of times._

// **Example 1:**

// **Input:** s = "1212"
// **Output:** 5
// **Explanation:** The substrings that meet the requirements are "1", "2", "12", "21", "1212".
// Note that although the substring "12" appears twice, it is only counted once.

// **Example 2:**

// **Input:** s = "12321"
// **Output:** 9
// **Explanation:** The substrings that meet the requirements are "1", "2", "3", "12", "23", "32", "21", "123", "321".

// **Constraints:**

// *   `1 <= s.length <= 1000`
// *   `s` consists of digits.

// **Companies**:
// [Expedia](https://leetcode.com/company/expedia)

// **Related Topics**:
// [Hash Table](https://leetcode.com/tag/hash-table/), [String](https://leetcode.com/tag/string/), [Rolling Hash](https://leetcode.com/tag/rolling-hash/), [Counting](https://leetcode.com/tag/counting/), [Hash Function](https://leetcode.com/tag/hash-function/)

// **Similar Questions**:

// *   [Number of Equal Count Substrings (Medium)](https://leetcode.com/problems/number-of-equal-count-substrings/)
// *   [Substrings That Begin and End With the Same Letter (Medium)](https://leetcode.com/problems/substrings-that-begin-and-end-with-the-same-letter/)

// Solution 1.[](https://leetcode.ca/2022-03-01-2168-Unique-Substrings-With-Equal-Digit-Frequency/#solution-1)
// -----------------------------------------------------------------------------------------------------------

//     // OJ: https://leetcode.com/problems/unique-substrings-with-equal-digit-frequency/
//     // Time: O(N^3)
//     // Space: O(N^2)
//     class Solution {
//     public:
//         int equalDigitFrequency(string s) {

#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn longest_word(words: Vec<String>) -> String {
        String::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_longest_word_1() {
        assert_eq!(
            "kiran".to_string(),
            Solution::longest_word(
                ["k", "ki", "kir", "kira", "kiran"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>()
            )
        );
    }
    #[test]
    pub fn test_longest_word_2() {
        assert_eq!(
            "apple".to_string(),
            Solution::longest_word(
                ["a", "banana", "app", "appl", "ap", "apply", "apple"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>()
            )
        );
    }
    #[test]
    pub fn test_longest_word_3() {
        assert_eq!(
            String::new(),
            Solution::longest_word(
                ["abc", "bc", "ab", "qwe"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>(),
            )
        );
    }
}
