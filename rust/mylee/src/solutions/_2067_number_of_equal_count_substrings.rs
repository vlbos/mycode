// # [2067. Number of Equal Count Substrings](https://leetcode.com/problems/number-of-equal-count-substrings)

// ## Description

// You are given a 0-indexed string s consisting of only lowercase English letters,
//  and an integer count. A substring of s is said to be an equal count substring if,
// for each unique letter in the substring, it appears exactly count times in the substring.

// Return the number of equal count substrings in s.

// A substring is a contiguous non-empty sequence of characters within a string.

// Example 1:

//
// Input: s = "aaabcbbcc", count = 3
// Output: 3
// Explanation:
// The substring that starts at index 0 and ends at index 2 is "aaa".
// The letter 'a' in the substring appears exactly 3 times.
// The substring that starts at index 3 and ends at index 8 is "bcbbcc".
// The letters 'b' and 'c' in the substring appear exactly 3 times.
// The substring that starts at index 0 and ends at index 8 is "aaabcbbcc".
// The letters 'a', 'b', and 'c' in the substring appear exactly 3 times.
//

// Example 2:

//
// Input: s = "abcd", count = 2
// Output: 0
// Explanation:
// The number of times each letter appears in s is less than count.
// Therefore, no substrings in s are equal count substrings, so return 0.
//

// Example 3:

//
// Input: s = "a", count = 5
// Output: 0
// Explanation:
// The number of times each letter appears in s is less than count.
// Therefore, no substrings in s are equal count substrings, so return 0

// Constraints:

//
// 	1 <= s.length <= 3 * 104
// 	1 <= count <= 3 * 104
// 	s consists only of lowercase English letters.
//

//    int equal_count_substrings(string s, int count) {

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn equal_count_substrings(s: String, count: i32) -> i32 {
        use std::collections::{HashMap, HashSet};
        let ss: HashSet<char> = s.chars().collect();
        let max = ss.len() as i32;
        let mut ans = 0;
        let bs = s.as_bytes();
        for n in 1..=max {
            let window_size = n * count;
            let mut cnt = HashMap::new();
            let mut unique_count = 0;
            for (i, c) in s.bytes().enumerate() {
                *cnt.entry(c).or_insert(0) += 1;
                if *cnt.get(&c).unwrap() == count {
                    unique_count += 1;
                }
                if i as i32 >= window_size {
                    cnt.entry(bs[i - window_size as usize])
                        .and_modify(|c| *c -= 1);
                    if *cnt.get(&bs[i - window_size as usize]).unwrap() == count - 1 {
                        unique_count -= 1;
                    }
                }
                if unique_count == n {
                    ans += 1;
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_equal_count_substrings_1() {
        assert_eq!(
            3,
            Solution::equal_count_substrings("aaabcbbcc".to_string(), 3)
        );
    }
    #[test]
    pub fn test_equal_count_substrings_2() {
        assert_eq!(0, Solution::equal_count_substrings("abcd".to_string(), 2));
    }
    #[test]
    pub fn test_equal_count_substrings_3() {
        assert_eq!(0, Solution::equal_count_substrings("a".to_string(), 5));
    }
}
