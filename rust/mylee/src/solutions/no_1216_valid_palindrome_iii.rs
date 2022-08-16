// 1216\. Valid Palindrome III
// ===========================

// Given a string `s` and an integer `k`, find out if the given string is a _K-Palindrome_ or not.

// A string is K-Palindrome if it can be transformed into a palindrome by removing at most `k` characters from it.

// **Example 1:**

// **Input:** s = "abcdeca", k = 2
// **Output:** true
// **Explanation:** Remove 'b' and 'e' characters.

// **Constraints:**

// *   `1 <= s.length <= 1000`
// *   `s` has only lowercase English letters.
// *   `1 <= k <= s.length`

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Facebook](https://leetcode.ca/tags/#Facebook)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn is_valid_palindrome(s: String, k: i32) -> bool {
        let k = k as usize;
        let n = s.len();
        if n <= k {
            return true;
        }
        let mut dp = vec![0; n];
        let bs = s.as_bytes();
        for i in (0..n).rev() {
            let mut new_dp = vec![0; n];
            new_dp[i] = 1;
            for j in i + 1..n {
                if bs[i] == bs[j] {
                    new_dp[j] = dp[j - 1] + 2;
                } else {
                    new_dp[j] = dp[j].max(new_dp[j - 1]);
                }
            }
            dp = new_dp;
        }
        n - dp[n - 1] <= k
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_is_valid_palindrome_1() {
        assert!(Solution::is_valid_palindrome(String::from("abcdeca"), 2));
    }
}
