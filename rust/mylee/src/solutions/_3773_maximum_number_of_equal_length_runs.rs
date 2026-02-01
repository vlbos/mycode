// # 3773. Maximum Number of Equal Length Runs 🔒

// Description
// -----------

// You are given a string `s` consisting of lowercase English letters.

// A **run** in `s` is a **substring** of **equal** letters that cannot be extended further.
// For example, the runs in `"hello"` are `"h"`, `"e"`, `"ll"`, and `"o"`.

// You can **select** runs that have the **same** length in `s`.

// Return an integer denoting the **maximum** number of runs you can select in `s`.

// **Example 1:**

// **Input:** s = "hello"

// **Output:** 3

// **Explanation:**

// The runs in `s` are `"h"`, `"e"`, `"ll"`, and `"o"`.
// You can select `"h"`, `"e"`, and `"o"` because they have the same length 1.

// **Example 2:**

// **Input:** s = "aaabaaa"

// **Output:** 2

// **Explanation:**

// The runs in `s` are `"aaa"`, `"b"`, and `"aaa"`.
// You can select `"aaa"` and `"aaa"` because they have the same length 3.

// **Constraints:**

// *   `1 <= s.length <= 105`
// *   `s` consists of lowercase English letters only.

// //  int max_same_length_runs(string s) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn max_same_length_runs(s: String) -> i32 {
        let mut cnt = std::collections::HashMap::new();
        let n = s.len();
        let mut cur = 0;
        for (i, c) in s.bytes().enumerate() {
            cur += 1;
            if i + 1 == n || c != s.as_bytes()[i + 1] {
                *cnt.entry(cur).or_insert(0) += 1;
                cur = 0;
            }
        }
        *cnt.values().max().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_max_same_length_runs_1() {
        assert_eq!(3, Solution::max_same_length_runs("hello".to_string()));
    }
    #[test]
    pub fn test_max_same_length_runs_2() {
        assert_eq!(2, Solution::max_same_length_runs("aaabaaa".to_string()));
    }
}
