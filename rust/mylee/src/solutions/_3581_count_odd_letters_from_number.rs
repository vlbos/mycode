// ## [3581\. Count Odd Letters from Number 🔒](https://leetcode.com/problems/count-odd-letters-from-number)

// [![](https://img.shields.io/badge/Difficulty-Easy-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Easy-4051B5?style=flat-square)

// ## Description

// You are given an integer `n` perform the following steps:

// +   Convert each digit of `n` into its *lowercase English word* (e.g., 4 → "four", 1 → "one").
// +   **Concatenate** those words in the **original digit order** to form a string `s`.

// Return the number of **distinct** characters in `s` that appear an **odd** number of times.

// **Example 1:**

// **Input:** n = 41

// **Output:** 5

// **Explanation:**

// 41 → `"fourone"`

// Characters with odd frequencies: `'f'`, `'u'`, `'r'`, `'n'`, `'e'`. Thus, the answer is 5.

// **Example 2:**

// **Input:** n = 20

// **Output:** 5

// **Explanation:**

// 20 → `"twozero"`

// Characters with odd frequencies: `'t'`, `'w'`, `'z'`, `'e'`, `'r'`. Thus, the answer is 5.

// **Constraints:**

// +   `1 <= n <= 109`

// int count_odd_letters(int n) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn count_odd_letters(n: i32) -> i32 {
        n.to_string()
            .bytes()
            .fold([0; 26], |mut s, b| {
                [
                    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                ][(b - b'0') as usize]
                    .bytes()
                    .for_each(|c| {
                        s[(c - b'a') as usize] += 1;
                    });
                s
            })
            .iter()
            .filter(|&&c| c % 2 != 0)
            .count() as _
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_count_odd_letters_1() {
        assert_eq!(5, Solution::count_odd_letters(41));
    }
    #[test]
    pub fn test_count_odd_letters_2() {
        assert_eq!(5, Solution::count_odd_letters(20));
    }
}
