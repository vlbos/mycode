/*
 * @lc app=leetcode id=2030 lang=rust
 *
 * [2030] Smallest K-Length Subsequence With Occurrences of a Letter
 */

// @lc code=start
impl Solution {
    pub fn smallest_subsequence(s: String, k: i32, letter: char, repetition: i32) -> String {
        let n = s.len();
        let mut cnt = s.chars().filter(|c| *c == letter).count() as i32;
        let mut m = n as i32 - k;
        let mut ans = String::new();
        let mut p = 0;
        for c in s.chars() {
            while m > 0 && !ans.is_empty() && c < ans.chars().last().unwrap() {
                if ans.chars().last().unwrap() == letter {
                    if repetition > p - 1 + cnt {
                        break;
                    }
                    p -= 1;
                }
                ans.pop();
                m -= 1;
            }
            if c == letter {
                p += 1;
                cnt -= 1;
            }
            ans.push(c);
        }
        while ans.len() > k as usize {
            if ans.chars().last().unwrap() == letter {
                p -= 1;
            }
            ans.pop();
        }
        let mut ans:Vec<char>=ans.chars().collect();
        for i in (0..ans.len()).rev() {
            if p < repetition && ans[i] != letter {
                ans[i] = letter;
                p += 1;
            }
        }
        ans.iter().collect()
    }
}
// @lc code=end
