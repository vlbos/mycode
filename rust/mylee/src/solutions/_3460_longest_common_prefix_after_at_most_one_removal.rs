// ## [3460\. Longest Common Prefix After at Most One Removal 🔒](https://leetcode.com/problems/longest-common-prefix-after-at-most-one-removal)

// [![](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)

// ## Description

// You are given two strings `s` and `t`.

// Return the **length** of the **longest common prefix** between `s` and `t` after removing **at most** one character from `s`.

// **Note:** `s` can be left without any removal.

// **Example 1:**

// **Input:** s = "madxa", t = "madam"

// **Output:** 4

// **Explanation:**

// Removing `s[3]` from `s` results in `"mada"`, which has a longest common prefix of length 4 with `t`.

// **Example 2:**

// **Input:** s = "leetcode", t = "eetcode"

// **Output:** 7

// **Explanation:**

// Removing `s[0]` from `s` results in `"eetcode"`, which matches `t`.

// **Example 3:**

// **Input:** s = "one", t = "one"

// **Output:** 3

// **Explanation:**

// No removal is needed.

// **Example 4:**

// **Input:** s = "a", t = "b"

// **Output:** 0

// **Explanation:**

// `s` and `t` cannot have a common prefix.

// **Constraints:**

// +   `1 <= s.length <= 105`
// +   `1 <= t.length <= 105`
// +   `s` and `t` contain only lowercase English letters.

//  int longest_common_prefix(string s, string t) {

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(s: String, t: String) -> i32 {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_longest_common_prefix_1() {
        assert_eq!(
            4,
            Solution::longest_common_prefix(String::from("madxa"), String::from("madam"))
        );
    }
    #[test]
    pub fn test_longest_common_prefix_2() {
        assert_eq!(
            7,
            Solution::longest_common_prefix(String::from("leetcode"), String::from("eetcode"))
        );
    }
    #[test]
    pub fn test_longest_common_prefix_3() {
        assert_eq!(
            3,
            Solution::longest_common_prefix(String::from("one"), String::from("one"))
        );
    }
    #[test]
    pub fn test_longest_common_prefix_4() {
        assert_eq!(
            0,
            Solution::longest_common_prefix(String::from("a"), String::from("b"))
        );
    }
}
