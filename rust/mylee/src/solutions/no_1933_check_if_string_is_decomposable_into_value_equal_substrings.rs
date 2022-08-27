// 1933\. Check if String Is Decomposable Into Value-Equal Substrings[](https://leetcode.ca/2021-08-06-1933-Check-if-String-Is-Decomposable-Into-Value-Equal-Substrings/#1933-check-if-string-is-decomposable-into-value-equal-substrings)
// =======================================================================================================================================================================================================================================

// Level[](https://leetcode.ca/2021-08-06-1933-Check-if-String-Is-Decomposable-Into-Value-Equal-Substrings/#level)
// ---------------------------------------------------------------------------------------------------------------

// Easy

// Description[](https://leetcode.ca/2021-08-06-1933-Check-if-String-Is-Decomposable-Into-Value-Equal-Substrings/#description)
// ---------------------------------------------------------------------------------------------------------------------------

// A **value-equal** string is a string where **all** characters are the same.

// *   For example, `"1111"` and `"33"` are value-equal strings.
// *   In contrast, `"123"` is not a value-equal string.

// Given a digit string `s`, decompose the string into some number of **consecutive value-equal** substrings where **exactly one** substring has a **length of** `2` and the remaining substrings have a **length of** `3`.

// Return _`true` if you can decompose `s` according to the above rules. Otherwise, return `false`_.

// A **substring** is a contiguous sequence of characters in a string.

// **Example 1:**

// **Input:** s = “000111000”

// **Output:** false

// **Explanation:** s cannot be decomposed according to the rules because \[“000”, “111”, “000”\] does not have a substring of length 2.

// **Example 2:**

// **Input:** s = “00011111222”

// **Output:** true

// **Explanation:** s can be decomposed into \[“000”, “111”, “11”, “222”\].

// **Example 3:**

// **Input:** s = “011100022233”

// **Output:** false

// **Explanation:** s cannot be decomposed according to the rules because of the first ‘0’.

// **Constraints:**

// *   `1 <= s.length <= 1000`
// *   `s` consists of only digits `'0'` through `'9'`.

// Solution[](https://leetcode.ca/2021-08-06-1933-Check-if-String-Is-Decomposable-Into-Value-Equal-Substrings/#solution)
// ---------------------------------------------------------------------------------------------------------------------

// Since exactly one substring of `s` has length 2 and the remaining substrings of `s` have lengths 3, the length of `s` must have remainder 2 when mod 3. If the remainder is not 2, return `false`.

// Then split `s` into several substrings, where each substring consists of all same characters, and any two adjacent substrings have different characters. There should be only one substring the length of which has remainder 2 when mod 3, and the remaining substrings must have lengths that are multiples of 3. Otherwise, return `false`.

// If the substrings meet the criteria above, return `true`.

//     class Solution {
//         public boolean isDecomposable(String s) {

#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn longest_word(
        words:Vec<String>,
    ) -> String {
       String::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
   
    #[test]
    pub fn test_longest_word_1() {
        assert_eq!("kiran".to_string(),Solution::longest_word(
            ["k","ki","kir","kira", "kiran"].into_iter().map(String::from).collect::<Vec<String>>()
        ));
    }
    #[test]
    pub fn test_longest_word_2() {
          assert_eq!("apple".to_string(),Solution::longest_word(
           ["a", "banana", "app", "appl", "ap", "apply", "apple"].into_iter().map(String::from).collect::<Vec<String>>()
        ));
    }
    #[test]
    pub fn test_longest_word_3() {
          assert_eq!(String::new(),Solution::longest_word(
            ["abc", "bc", "ab", "qwe"].into_iter().map(String::from).collect::<Vec<String>>(),
        ));
    }
}
