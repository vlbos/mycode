// 1698\. Number of Distinct Substrings in a String
// ================================================

// Given a string `s`, return _the number of **distinct** substrings of_ `s`.

// A **substring** of a string is obtained by deleting any number of characters (possibly zero)
// from the front of the string and any number (possibly zero) from the back of the string.

// **Example 1:**

// **Input:** s = "aabbaba"
// **Output:** 21
// **Explanation:** The set of distinct strings is \["a","b","aa","bb","ab","ba","aab","abb","bba","aba","aabb","abba","bbab","baba","aabba","abbab","bbaba","aabbab","abbaba","aabbaba"\]

// **Example 2:**

// **Input:** s = "abcdefg"
// **Output:** 28

// **Constraints:**

// *   `1 <= s.length <= 500`
// *   `s` consists of lowercase English letters.

// **Follow up:** Can you solve this problem in `O(n)` time complexity?

// **Hints:**

// Hint 1

// Calculate the prefix hashing array for s.

// Hint 2

// Use the prefix hashing array to calculate the hashing value of each substring.

// Hint 3

// Compare the hashing values to determine the unique substrings.

// Hint 4

// There could be collisions if you use hashing, what about double hashing.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Dunzo](https://leetcode.ca/tags/#Dunzo) [Intuit](https://leetcode.ca/tags/#Intuit)

//  int count_distinct(String s)

#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn count_distinct(s: String) -> i32 {
        let mut ss = std::collections::HashSet::new();
        let n = s.len();
        for i in 0..n {
            for j in i + 1..=n {
                ss.insert(s[i..j].to_string());
            }
        }
        ss.len() as _
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_count_distinct_1() {
        assert_eq!(21, Solution::count_distinct(String::from("aabbaba")));
    }
    #[test]
    pub fn test_count_distinct_2() {
        assert_eq!(28, Solution::count_distinct(String::from("abcdefg")));
    }
}
