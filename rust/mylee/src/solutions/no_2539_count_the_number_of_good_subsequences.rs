// # [2539. Count the Number of Good Subsequences](https://leetcode.com/problems/count-the-number-of-good-subsequences)
// ## Description

//  A  subsequence  of a string is good if it is not empty and the frequency of each one of its characters is the same.

//  Given a string  s , return  the number of good subsequences of   s . Since the answer may be too large, return it modulo  10 9  + 7 .

//  A  subsequence  is a string that can be derived from another string by deleting some or no characters without changing the order of the remaining characters.

//  Example 1:

//  Input:  s = "aabb"
//  Output:  11
//  Explanation:  The total number of subsequences is  2 4 .  There are five subsequences which are not good: "  aab  b", "a  abb  ", "  a  a  bb  ", "  aa  b  b  ", and the empty subsequence. Hence, the number of good subsequences is  2 4 -5 = 11 .

//  Example 2:

//  Input:  s = "leet"
//  Output:  12
//  Explanation:  There are four subsequences which are not good: "  l  ee  t", "l  eet  ", "  leet  ", and the empty subsequence. Hence, the number of good subsequences is  2 4 -4 = 12 .

//  Example 3:

//  Input:  s = "abcd"
//  Output:  15
//  Explanation:  All of the non-empty subsequences are good subsequences. Hence, the number of good subsequences is  2 4 -1 = 15 .

//   Constraints:

// 	  1 <= s.length <= 10^4
// 	  s  consists of only lowercase English letters.
//  int count_good_subsequences(string s) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn count_good_subsequences(s: String) -> i32 {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_count_good_subsequences_1() {
        assert_eq!(11, Solution::count_good_subsequences("aabb".to_string()));
    }
    #[test]
    pub fn test_count_good_subsequences_2() {
        assert_eq!(12, Solution::count_good_subsequences("leet".to_string()));
    }
    #[test]
    pub fn test_count_good_subsequences_3() {
        assert_eq!(15, Solution::count_good_subsequences("abcd".to_string()));
    }
}
