// # 3758. Convert Number Words to Digits 🔒

// Description
// -----------

// You are given a string `s` consisting of lowercase English letters.
// `s` may contain **valid concatenated** English words representing the digits 0 to 9, without spaces.

// Your task is to **extract** each valid number word **in order** and convert it to its corresponding digit,
// producing a string of digits.

// Parse `s` from left to right. At each position:

// *   If a valid number word starts at the current position,
// append its corresponding digit to the result and advance by the length of that word.
// *   Otherwise, skip **exactly** one character and continue parsing.

// Return the resulting digit string. If no number words are found, return an empty string.

// **Example 1:**

// **Input:** s = "onefourthree"

// **Output:** "143"

// **Explanation:**

// *   Parsing from left to right, extract the valid number words "one", "four", "three".
// *   These map to digits 1, 4, 3. Thus, the final result is `"143"`.

// **Example 2:**

// **Input:** s = "ninexsix"

// **Output:** "96"

// **Explanation:**

// *   The substring `"nine"` is a valid number word and maps to 9.
// *   The character `"x"` does not match any valid number word prefix and is skipped.
// *   Then, the substring `"six"` is a valid number word and maps to 6, so the final result is `"96"`.

// **Example 3:**

// **Input:** s = "zeero"

// **Output:** ""

// **Explanation:**

// *   No substring forms a valid number word during left-to-right parsing.
// *   All characters are skipped and incomplete fragments are ignored, so the result is an empty string.

// **Example 4:**

// **Input:** s = "tw"

// **Output:** ""

// **Explanation:**

// *   No substring forms a valid number word during left-to-right parsing.
// *   All characters are skipped and incomplete fragments are ignored, so the result is an empty string.

// **Constraints:**

// *   `1 <= s.length <= 105`
// *   `s` contains only lowercase English letters.

// //  string convert_number(string s) {

impl Solution {
    pub fn convert_number(s: String) -> String {
        use std::collections::{HashMap, HashSet};
        let h: HashMap<_, _> = [
            "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ]
        .into_iter()
        .enumerate()
        .map(|(i, x)| (x, i))
        .collect();
        let mut pre = h.keys().fold(HashSet::new(), |mut p, w| {
            (1..=w.len()).for_each(|i| {
                p.insert(&w[..i]);
            });
            p
        });
        let (mut cur, mut ans) = (String::new(), String::new());
        for c in s.chars() {
            cur.push(c);
            if !pre.contains(&cur.as_str()) {
                cur = String::new();
                continue;
            }
            if let Some(v) = h.get(&cur.as_str()) {
                ans.push_str(&v.to_string());
                cur = String::new();
            }
        }
        ans
    }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_convert_number_1() {
        assert_eq!(
            Solution::convert_number(String::from("onefourthree")),
            "143"
        );
    }

    #[test]
    pub fn test_convert_number_2() {
        assert_eq!(Solution::convert_number(String::from("ninexsix")), "96");
    }

    #[test]
    pub fn test_convert_number_3() {
        assert_eq!(Solution::convert_number(String::from("zeero")), "");
    }
    #[test]
    pub fn test_convert_number_4() {
        assert_eq!(Solution::convert_number(String::from("tw")), "");
    }
}
