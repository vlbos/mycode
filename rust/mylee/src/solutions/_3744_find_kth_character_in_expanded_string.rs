// # 3744. Find Kth Character in Expanded String 🔒

// Description
// -----------

// You are given a string `s` consisting of one or more words separated by single spaces. Each word in `s` consists of lowercase English letters.

// We obtain the **expanded** string `t` from `s` as follows:

// *   For each **word** in `s`, repeat its first character once, then its second character twice, and so on.

// For example, if `s = "hello world"`, then `t = "heelllllllooooo woorrrllllddddd"`.

// You are also given an integer `k`, representing a **valid** index of the string `t`.

// Return the `kth` character of the string `t`.

// **Example 1:**

// **Input:** s = "hello world", k = 0

// **Output:** "h"

// **Explanation:**

// `t = "heelllllllooooo woorrrllllddddd"`. Therefore, the answer is `t[0] = "h"`.

// **Example 2:**

// **Input:** s = "hello world", k = 15

// **Output:** " "

// **Explanation:**

// `t = "heelllllllooooo woorrrllllddddd"`. Therefore, the answer is `t[15] = " "`.

// **Constraints:**

// *   `1 <= s.length <= 105`
// *   `s` contains only lowercase English letters and spaces `' '`.
// *   `s` **does not contain** any leading or trailing spaces.
// *   All the words in `s` are separated by a **single space**.
// *   `0 <= k < t.length`. That is, `k` is a **valid** index of `t`.

// //  char kth_character(string s, long long k) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn kth_character(s: String, mut k: i64) -> char {
        for w in s.split_ascii_whitespace() {
            let m = w.len() as i64;
            let n = (m + 1) * m / 2;
            if k == n {
                return ' ';
            }
            if k > n {
                k -= n + 1;
                continue;
            }
            let mut cnt = 0;
            for (i, c) in w.chars().enumerate() {
                cnt += i as i64 + 1;
                if k < cnt {
                    return c;
                }
            }
        }
        ' '
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_kth_character_1() {
        assert_eq!(Solution::kth_character(String::from("hello world"), 0), 'h');
    }

    #[test]
    pub fn test_kth_character_2() {
        assert_eq!(
            Solution::kth_character(String::from("hello world"), 15),
            ' '
        );
    }
}
