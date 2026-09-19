// ## [3571\. Find the Shortest Superstring II 🔒](https://leetcode.com/problems/find-the-shortest-superstring-ii)

// [![](https://img.shields.io/badge/Difficulty-Easy-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Easy-4051B5?style=flat-square)

// ## Description

// You are given **two** strings, `s1` and `s2`.
// Return the **shortest** *possible* string that contains both `s1` and `s2` as substrings.
// If there are multiple valid answers, return *any* one of them.

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
        if s2.contains(&s1) {
            return s2;
        }
        if s1.contains(&s2) {
            return s1;
        }
        let max_overlap = |s1: &str, s2: &str| {
            let (len1, len2) = (s1.len(), s2.len());
            for i in (1..=len1.min(len2)).rev() {
                if &s1[len1 - i..] == &s2[..i] {
                    return i;
                }
            }
            0
        };
        let (overlap1, overlap2) = (max_overlap(&s1, &s2), max_overlap(&s2, &s1));
        if overlap1 >= overlap2 {
            s1 + &s2[overlap1..]
        } else {
            s2 + &s1[overlap2..]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_shortest_superstring_1() {
        assert_eq!(
            "abab".len(),
            Solution::shortest_superstring("aba".to_owned(), "bab".to_owned(),).len()
        );
    }
    #[test]
    pub fn test_shortest_superstring_2() {
        assert_eq!(
            "aaa".len(),
            Solution::shortest_superstring("aa".to_owned(), "aaa".to_owned(),).len()
        );
    }
}
