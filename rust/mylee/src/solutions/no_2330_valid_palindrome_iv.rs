// # [2330. Valid Palindrome IV](https://leetcode.com/problems/valid-palindrome-iv)

// ## Description

// You are given a 0-indexed string s consisting of only lowercase English letters. In one operation, you can change any character of s to any other character.

// Return true if you can make s a palindrome after performing exactly one or two operations, or return false otherwise.

// Example 1:

//
// Input: s = &quot;abcdba&quot;
// Output: true
// Explanation: One way to make s a palindrome using 1 operation is:
// - Change s[2] to 'd'. Now, s = &quot;abddba&quot;.
// One operation could be performed to make s a palindrome so return true.
//

// Example 2:

//
// Input: s = &quot;aa&quot;
// Output: true
// Explanation: One way to make s a palindrome using 2 operations is:
// - Change s[0] to 'b'. Now, s = &quot;ba&quot;.
// - Change s[1] to 'b'. Now, s = &quot;bb&quot;.
// Two operations could be performed to make s a palindrome so return true.
//

// Example 3:

//
// Input: s = &quot;abcdef&quot;
// Output: false
// Explanation: It is not possible to make s a palindrome using one or two operations so return false.
//

// Constraints:

//
// 	1 <= s.length <= 105
// 	s consists only of lowercase English letters.
//

// ## Solutions

// <!-- tabs:start -->

// ### **Python3**

// ```python
// class Solution:
//     def makePalindrome(self, s: str) -> bool:

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
