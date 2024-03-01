// [2083\. Substrings That Begin and End With the Same Letter (Medium)](https://leetcode.com/problems/substrings-that-begin-and-end-with-the-same-letter/)[](https://leetcode.ca/2021-12-06-2083-Substrings-That-Begin-and-End-With-the-Same-Letter/#2083-substrings-that-begin-and-end-with-the-same-letter-medium)
// =================================================================================================================================================================================================================================================================================================================

// You are given a **0-indexed** string `s` consisting of only lowercase English letters.
// Return _the number of **substrings** in_ `s` _that begin and end with the **same** character._

// A **substring** is a contiguous non-empty sequence of characters within a string.

// **Example 1:**

// **Input:** s = "abcba"
// **Output:** 7
// **Explanation:**
// The substrings of length 1 that start and end with the same letter are: "a", "b", "c", "b", and "a".
// The substring of length 3 that starts and ends with the same letter is: "bcb".
// The substring of length 5 that starts and ends with the same letter is: "abcba".

// **Example 2:**

// **Input:** s = "abacad"
// **Output:** 9
// **Explanation:**
// The substrings of length 1 that start and end with the same letter are: "a", "b", "a", "c", "a", and "d".
// The substrings of length 3 that start and end with the same letter are: "aba" and "aca".
// The substring of length 5 that starts and ends with the same letter is: "abaca".

// **Example 3:**

// **Input:** s = "a"
// **Output:** 1
// **Explanation:**
// The substring of length 1 that starts and ends with the same letter is: "a".

// **Constraints:**

// *   `1 <= s.length <= 105`
// *   `s` consists only of lowercase English letters.

// **Companies**:
// [Google](https://leetcode.com/company/google)

// **Related Topics**:
// [Hash Table](https://leetcode.com/tag/hash-table/), [Math](https://leetcode.com/tag/math/), [String](https://leetcode.com/tag/string/), [Counting](https://leetcode.com/tag/counting/), [Prefix Sum](https://leetcode.com/tag/prefix-sum/)

// **Similar Questions**:

// *   [Number of Good Pairs (Easy)](https://leetcode.com/problems/number-of-good-pairs/)
// *   [Sum of Beauty of All Substrings (Medium)](https://leetcode.com/problems/sum-of-beauty-of-all-substrings/)
// *   [Count Pairs in Two Arrays (Medium)](https://leetcode.com/problems/count-pairs-in-two-arrays/)

// Solution 1.[](https://leetcode.ca/2021-12-06-2083-Substrings-That-Begin-and-End-With-the-Same-Letter/#solution-1)
// -----------------------------------------------------------------------------------------------------------------

//     // OJ: https://leetcode.com/problems/substrings-that-begin-and-end-with-the-same-letter/
//     // Time: O(N)
//     // Space: O(1)
//     class Solution {
//     public:
//         long long number_of_substrings(string s) {

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn number_of_substrings(s: String) -> i64 {
        let mut ans = 0;
        let mut cnt = std::collections::HashMap::new();
        for c in s.chars() {
            *cnt.entry(c).or_insert(0) += 1;
            ans += cnt[&c];
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_number_of_substrings_1() {
        assert_eq!(7, Solution::number_of_substrings("abcba".to_string()));
    }
    #[test]
    pub fn test_number_of_substrings_2() {
        assert_eq!(9, Solution::number_of_substrings("abacad".to_string()));
    }
    #[test]
    pub fn test_number_of_substrings_3() {
        assert_eq!(1, Solution::number_of_substrings("a".to_string()));
    }
}
