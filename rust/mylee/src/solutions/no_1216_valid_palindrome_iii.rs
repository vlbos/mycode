/*
Given a string s and an integer k, find out if the given string is a K-Palindrome or not.

A string is K-Palindrome if it can be transformed into a palindrome by removing at most k characters from it.


Example 1:
Input: s = "abcdeca", k = 2
Output: true
Explanation: Remove 'b' and 'e' characters.


Constraints:
    1 <= s.length <= 1000
    s has only lowercase English letters.
    1 <= k <= s.length


*/
#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn   is_valid_palindrome(s: String, k: i32) -> bool {
        true
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
   pub fn  test_is_valid_palindrome_1() {
        assert!(Solution::is_valid_palindrome(String::new(), 0));
    }
}
