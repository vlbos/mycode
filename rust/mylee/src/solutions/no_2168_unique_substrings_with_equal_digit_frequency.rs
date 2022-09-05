// [2168\. Unique Substrings With Equal Digit Frequency (Medium)](https://leetcode.com/problems/unique-substrings-with-equal-digit-frequency/)[](https://leetcode.ca/2022-03-01-2168-Unique-Substrings-With-Equal-Digit-Frequency/#2168-unique-substrings-with-equal-digit-frequency-medium)
// =========================================================================================================================================================================================================================================================================================

// Given a digit string `s`, return _the number of **unique substrings** of_ `s` _where every digit appears the same number of times._

// **Example 1:**

// **Input:** s = "1212"
// **Output:** 5
// **Explanation:** The substrings that meet the requirements are "1", "2", "12", "21", "1212".
// Note that although the substring "12" appears twice, it is only counted once.

// **Example 2:**

// **Input:** s = "12321"
// **Output:** 9
// **Explanation:** The substrings that meet the requirements are "1", "2", "3", "12", "23", "32", "21", "123", "321".

// **Constraints:**

// *   `1 <= s.length <= 1000`
// *   `s` consists of digits.

// **Companies**:
// [Expedia](https://leetcode.com/company/expedia)

// **Related Topics**:
// [Hash Table](https://leetcode.com/tag/hash-table/), [String](https://leetcode.com/tag/string/), [Rolling Hash](https://leetcode.com/tag/rolling-hash/), [Counting](https://leetcode.com/tag/counting/), [Hash Function](https://leetcode.com/tag/hash-function/)

// **Similar Questions**:

// *   [Number of Equal Count Substrings (Medium)](https://leetcode.com/problems/number-of-equal-count-substrings/)
// *   [Substrings That Begin and End With the Same Letter (Medium)](https://leetcode.com/problems/substrings-that-begin-and-end-with-the-same-letter/)

// Solution 1.[](https://leetcode.ca/2022-03-01-2168-Unique-Substrings-With-Equal-Digit-Frequency/#solution-1)
// -----------------------------------------------------------------------------------------------------------

//     // OJ: https://leetcode.com/problems/unique-substrings-with-equal-digit-frequency/
//     // Time: O(N^3)
//     // Space: O(N^2)
//     class Solution {
//     public:
//         int equal_digit_frequency(string s) {

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn equal_digit_frequency(s: String) -> i32 {
        let mut ans = std::collections::HashSet::new();
        for (i, b) in s.bytes().enumerate() {
            let (mut hash, mut max, mut unique) = (0, 0, 0);
            let mut cnt = vec![0; 10];
            for (j, bj) in s[i..].bytes().enumerate() {
                let d = (bj - b'0') as usize;
                cnt[d] += 1;
                if cnt[d] == 1 {
                    unique += 1;
                }
                max = max.max(cnt[d]);
                hash = (hash * 31 + (d as i64) + 1) % 1_000_000_007;
                if max * unique == (j + 1) as i64 {
                    ans.insert(hash);
                }
            }
        }
        ans.len() as _
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_equal_digit_frequency_1() {
        assert_eq!(5, Solution::equal_digit_frequency("1212".to_string()));
    }
    #[test]
    pub fn test_equal_digit_frequency_2() {
        assert_eq!(9, Solution::equal_digit_frequency("12321".to_string()));
    }
}
