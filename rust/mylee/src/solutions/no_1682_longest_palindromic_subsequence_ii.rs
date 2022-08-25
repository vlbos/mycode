// 1682\. Longest Palindromic Subsequence II
// =========================================

// A subsequence of a string `s` is considered a **good palindromic subsequence** if:

// *   It is a subsequence of `s`.
// *   It is a palindrome (has the same value if reversed).
// *   It has an **even** length.
// *   No two consecutive characters are equal, except the two middle ones.

// For example, if `s = "abcabcabb"`, then `"abba"` is considered a **good palindromic subsequence**,
// while `"bcb"` (not even length) and `"bbbb"` (has equal consecutive characters) are not.

// Given a string `s`, return _the **length** of the **longest good palindromic subsequence** in_ `s`.

// **Example 1:**

// **Input:** s = "bbabab"
// **Output:** 4
// **Explanation:** The longest good palindromic subsequence of s is "baab".

// **Example 2:**

// **Input:** s = "dcbccacdb"
// **Output:** 4
// **Explanation:** The longest good palindromic subsequence of s is "dccd".

// **Constraints:**

// *   `1 <= s.length <= 250`
// *   `s` consists of lowercase English letters.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Codenation](https://leetcode.ca/tags/#Codenation)

// int longest_palindrome_subseq(String s)

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let n = s.len();
        let mut dp = vec![vec![vec![0; 27]; n]; n];
        let bs = s.as_bytes();
        for d in 1..n {
            for i in 0..n - d {
                for k in 0..=26 {
                    let j = i + d;
                    dp[i][j][k] = if bs[i] == bs[j] && bs[i] != b'a' + k as u8 {
                        dp[i + 1][j - 1][(bs[i] - b'a') as usize] + 2
                    } else {
                        dp[i + 1][j][k].max(dp[i][j - 1][k])
                    };
                }
            }
        }
        dp[0][n - 1][26]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_longest_palindrome_subseq_1() {
        assert_eq!(
            4,
            Solution::longest_palindrome_subseq(String::from("bbabab"))
        );
    }
    #[test]
    pub fn test_longest_palindrome_subseq_2() {
        assert_eq!(
            4,
            Solution::longest_palindrome_subseq(String::from("dcbccacdb"))
        );
    }
}
