// 1794\. Count Pairs of Equal Substrings With Minimum Difference
// ==============================================================

// You are given two strings `firstString` and `secondString` that are **0-indexed** and consist only of lowercase English letters.
// Count the number of index quadruples `(i,j,a,b)` that satisfy the following conditions:

// *   `0 <= i <= j < firstString.length`
// *   `0 <= a <= b < secondString.length`
// *   The substring of `firstString` that starts at the `ith` character and ends at the `jth` character (inclusive) is **equal** to the substring of `secondString` that starts at the `ath` character and ends at the `bth` character (inclusive).
// *   `j - a` is the **minimum** possible value among all quadruples that satisfy the previous conditions.

// Return _the **number** of such quadruples_.

// **Example 1:**

// **Input:** firstString = "abcd", secondString = "bccda"
// **Output:** 1
// **Explanation:** The quadruple (0,0,4,4) is the only one that satisfies all the conditions and minimizes j - a.

// **Example 2:**

// **Input:** firstString = "ab", secondString = "cd"
// **Output:** 0
// **Explanation:** There are no quadruples satisfying all the conditions.

// **Constraints:**

// *   `1 <= firstString.length, secondString.length <= 2 * 105`
// *   Both strings consist only of lowercase English letters.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

//  int count_quadruples(string first_string, string second_string)

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn count_quadruples(first_string: String, second_string: String) -> i32 {
        let mut m = std::collections::HashMap::new();
        for (i, c) in second_string.chars().enumerate() {
            m.insert(c, i as i32);
        }
        let mut min = i32::MAX;
        let mut cnt = 0;
        for (j, c) in first_string.chars().enumerate() {
            if let Some(&a) = m.get(&c) {
                let ja = j as i32 - a;
                if ja < min {
                    min = ja;
                    cnt = 1;
                } else if ja == min {
                    cnt += 1;
                }
            }
        }
        cnt
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_count_quadruples_1() {
        assert_eq!(
            1,
            Solution::count_quadruples(String::from("abcd"), String::from("bccda"))
        );
    }
    #[test]
    pub fn test_count_quadruples_2() {
        assert_eq!(
            0,
            Solution::count_quadruples(String::from("ab"), String::from("cd"))
        );
    }
}
