// # [2950. Number of Divisible Substrings](https://leetcode.com/problems/number-of-divisible-substrings)

// ## Description

// Each character of the English alphabet has been mapped to a digit as shown below.

// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2900-2999/2950.Number%20of%20Divisible%20Substrings/images/old_phone_digits.png" style="padding: 10px; width: 200px; height: 200px;" />

// A string is divisible if the sum of the mapped values of its characters is divisible by its length.

// Given a string s, return the number of divisible substrings of s.

// A substring is a contiguous non-empty sequence of characters within a string.

//
// Example 1:
//|Substring|Mapped|Sum|Length|Divisible?|
// |a|1|1|1|Yes|
// |s|7|7|1|Yes|
// |d|2|2|1|Yes|
// |f|3|3|1|Yes|
// |as|1, 7|8|2|Yes|
// |sd|7, 2|9|2|No|
// |df|2, 3|5|2|No|
// |asd|1, 7, 2|10|3|No|
// |sdf|7, 2, 3|12|3|Yes|
// |asdf|1, 7, 2, 3|13|4|No|

// Input: word = "asdf"
// Output: 6
// Explanation: The table above contains the details about every substring of word,
// and we can see that 6 of them are divisible.

// Example 2:

// Input: word = "bdh"
// Output: 4
// Explanation: The 4 divisible substrings are: "b", "d", "h", "bdh".
// It can be shown that there are no other substrings of word that are divisible.

// Example 3:

// Input: word = "abcd"
// Output: 6
// Explanation: The 6 divisible substrings are: "a", "b", "c", "d", "ab", "cd".
// It can be shown that there are no other substrings of word that are divisible.

//
// Constraints:

// 	1  <= word.length  <= 2000
// 	word consists only of lowercase English letters.

// ```rust
// use std::collections::HashMap;

// impl Solution {
//     pub fn count_divisible_substrings(word: String) -> i32 {
//
//     }
// }
// ```

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn count_divisible_substrings(word: String) -> i32 {
        let c2i = |b: u8| {
            let i = (b - b'a') as i32 + 1;
            i / 3 + 1
        };
        let mut ans = 0;
        for i in 1..10 {
            let mut cnt = std::collections::HashMap::from([(0, 1)]);
            let mut sum = 0;
            for b in word.bytes() {
                sum += c2i(b) - i;
                if let Some(&v) = cnt.get(&sum) {
                    ans += v;
                }
                *cnt.entry(sum).or_default() += 1;
            }
        }
        ans
    }
    pub fn count_divisible_substrings1(word: String) -> i32 {
        let c2i = |b: u8| {
            let i = (b - b'a') as i32 + 1;
            i / 3 + 1
        };
        let w = word.as_bytes();
        let n = w.len();
        let mut ans = 0;
        for i in 0..n {
            let mut sum = 0;
            for j in i..n {
                sum += c2i(w[j]);
                if sum % ((j - i + 1) as i32) == 0 {
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
    pub fn test_count_divisible_substrings_1() {
        assert_eq!(6, Solution::count_divisible_substrings("asdf".to_string()));
    }
    #[test]
    pub fn test_count_divisible_substrings_2() {
        assert_eq!(4, Solution::count_divisible_substrings("bdh".to_string()));
    }
    #[test]
    pub fn test_count_divisible_substrings_3() {
        assert_eq!(6, Solution::count_divisible_substrings("abcd".to_string()));
    }
}
