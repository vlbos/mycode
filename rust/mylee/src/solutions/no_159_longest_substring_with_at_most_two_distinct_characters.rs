// 159. Longest Substring with At Most Two Distinct Characters
// Given a string s , find the length of the longest substring t  that contains at most 2 distinct characters.

// Example 1:

// Input: "eceba"
// Output: 3
// Explanation: t is "ece" which its length is 3.
// Example 2:

// Input: "ccaabbb"
// Output: 5
// Explanation: t is "aabbb" which its length is 5.
// Difficulty:
// Medium
// Lock:
// Prime
// Company:
// Adobe Amazon Facebook Google Microsoft
struct Solution;
// @lc code=start

use std::mem::swap;

impl Solution {
    pub fn length_of_longest_substring_two_distinct(s: String) -> i32 {
        let chars = s.chars().collect::<Vec<char>>();
        if chars.len() <= 2 {
            return chars.len() as i32;
        }
        let mut diff1 = chars[0];
        let mut diff2 = chars[1];
        let mut max_len = 2;
        let mut current_len = 2;
        let mut last_len = if diff1 == diff2 { 2 } else { 1 };
        for i in 2..chars.len() {
            let current = chars[i];
            if current == diff2 {
                current_len += 1;
                last_len += 1;
            } else if current == diff1 {
                current_len += 1;
                swap(&mut diff1, &mut diff2);
                last_len = 1;
            } else {
                diff1 = diff2;
                diff2 = current;
                current_len = last_len + 1;
                last_len = 1;
            }
            max_len = i32::max(max_len, current_len);
        }
        max_len
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring_two_distinct() {
        assert_eq!(
            Solution::length_of_longest_substring_two_distinct(String::from("eceba")),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring_two_distinct(String::from("ccaabbb")),
            5
        );
    }
}
