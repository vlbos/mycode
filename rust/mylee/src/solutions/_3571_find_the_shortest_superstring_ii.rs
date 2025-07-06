// ## [3571\. Find the Shortest Superstring II 🔒](https://leetcode.com/problems/find-the-shortest-superstring-ii)

// [![](https://img.shields.io/badge/Difficulty-Easy-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Easy-4051B5?style=flat-square)

// ## Description

// You are given **two** strings, `s1` and `s2`. Return the **shortest** *possible* string that contains both `s1` and `s2` as substrings. If there are multiple valid answers, return *any* one of them.

// A **substring** is a contiguous sequence of characters within a string.

// **Example 1:**

// **Input:** s1 = "aba", s2 = "bab"

// **Output:** "abab"

// **Explanation:**

// `"abab"` is the shortest string that contains both `"aba"` and `"bab"` as substrings.

// **Example 2:**

// **Input:** s1 = "aa", s2 = "aaa"

// **Output:** "aaa"

// **Explanation:**

// `"aa"` is already contained within `"aaa"`, so the shortest superstring is `"aaa"`.

// **Constraints:**

// +   `1 <= s1.length <= 100`
// +   `1 <= s2.length <= 100`
// +   `s1` and `s2` consist of lowercase English letters only.

//  string shortest_superstring(string s1, string s2) {

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn shortest_superstring(s1: String, s2: String) -> String {
        String::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_shortest_superstring_1() {
        assert_eq!(
            "abab".to_owned(),
            Solution::shortest_superstring(
                "aba".to_owned(),
                "bab".to_owned(),
            )
        );
    }
    #[test]
    pub fn test_shortest_superstring_2() {
        assert_eq!(
            "aaa".to_owned(),
            Solution::shortest_superstring(
                "aa".to_owned(),
                "aaa".to_owned(),
            )
        );
    }
    
}
