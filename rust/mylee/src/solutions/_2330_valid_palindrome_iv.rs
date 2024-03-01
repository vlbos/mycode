// # [2330. Valid Palindrome IV](https://leetcode.com/problems/valid-palindrome-iv)

// ## Description

// You are given a 0-indexed string s consisting of only lowercase English letters.
// In one operation, you can change any character of s to any other character.

// Return true if you can make s a palindrome after performing exactly one or two operations, or return false otherwise.

// Example 1:

//
// Input: s = "abcdba"
// Output: true
// Explanation: One way to make s a palindrome using 1 operation is:
// - Change s[2] to 'd'. Now, s = "abddba".
// One operation could be performed to make s a palindrome so return true.
//

// Example 2:

//
// Input: s = "aa"
// Output: true
// Explanation: One way to make s a palindrome using 2 operations is:
// - Change s[0] to 'b'. Now, s = "ba".
// - Change s[1] to 'b'. Now, s = "bb".
// Two operations could be performed to make s a palindrome so return true.
//

// Example 3:

//
// Input: s = "abcdef"
// Output: false
// Explanation: It is not possible to make s a palindrome using one or two operations so return false.
//

// Constraints:

//
// 	1 <= s.length <= 10^5
// 	s consists only of lowercase English letters.
//

// ## Solutions

// <!-- tabs:start -->

// ### **Python3**

// ```python
// class Solution:
//     def make_palindrome(self, s: str) -> bool:

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn make_palindrome(s: String) -> bool {
        let mut ans = 0;
        let bs = s.as_bytes();
        for i in 0..bs.len() / 2 {
            if bs[i] != bs[bs.len() - 1 - i] {
                ans += 1;
            }
            if ans > 2 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_make_palindrome_1() {
        assert!(Solution::make_palindrome("abcdba".to_string()));
    }
    #[test]
    pub fn test_make_palindrome_2() {
        assert!(Solution::make_palindrome("aa".to_string()));
    }
    #[test]
    pub fn test_make_palindrome_3() {
        assert!(!Solution::make_palindrome("abcdef".to_string()));
    }
}
