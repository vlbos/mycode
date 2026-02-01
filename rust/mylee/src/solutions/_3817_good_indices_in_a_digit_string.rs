// [3817\. Good Indices in a Digit String 🔒](https://leetcode.com/problems/good-indices-in-a-digit-string)
// ========================================================================================================

// [![](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)

// Description
// -----------

// You are given a string `s` consisting of digits.

// An index `i` is called **good** if there exists a substring of `s` that ends at index `i` and is equal to the decimal representation of `i`.

// Return an integer array of all good indices in **increasing order**.

// **Example 1:**

// **Input:** s = "0234567890112"

// **Output:** \[0,11,12\]

// **Explanation:​​​​​​​**

// *   At index 0, the decimal representation of the index is `"0"`. The substring `s[0]` is `"0"`, which matches, so index `0` is good.

// *   At index 11, the decimal representation is `"11"`. The substring `s[10..11]` is `"11"`, which matches, so index `11` is good.

// *   At index 12, the decimal representation is `"12"`. The substring `s[11..12]` is `"12"`, which matches, so index `12` is good.

// No other index has a substring ending at it that equals its decimal representation. Therefore, the answer is `[0, 11, 12]`.

// **Example 2:**

// **Input:** s = "01234"

// **Output:** \[0,1,2,3,4\]

// **Explanation:**

// For every index `i` from 0 to 4, the decimal representation of `i` is a single digit, and the substring `s[i]` matches that digit.

// Therefore, a valid substring ending at each index exists, making all indices good.

// **Example 3:**

// **Input:** s = "12345"

// **Output:** \[\]

// **Explanation:**

// No index has a substring ending at it that matches its decimal representation.

// Therefore, there are no good indices and the result is an empty array.

// **Constraints:**

// *   `1 <= s.length <= 105`
// *   `s` only consists of digits from `'0'` to `'9'`.

// vector<int> good_indices(string s) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn good_indices(s: String) -> Vec<i32> {
        vec![]
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_good_indices_1() {
        assert_eq!(
            Solution::good_indices("0234567890112".to_owned()),
            vec![0, 11, 12]
        );
    }

    #[test]
    pub fn test_good_indices_2() {
        assert_eq!(
            Solution::good_indices("01234".to_owned()),
            vec![0, 1, 2, 3, 4]
        );
    }

    #[test]
    pub fn test_good_indices_3() {
        assert_eq!(Solution::good_indices("12345".to_owned()), vec![]);
    }
}
