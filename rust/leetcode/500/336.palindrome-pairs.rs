/*
 * @lc app=leetcode id=336 lang=rust
 *
 * [336] Palindrome Pairs
 */

// @lc code=start
impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
       use std::collections::HashMap;
        let w2i: HashMap<String, usize> = words
            .iter()
            .enumerate()
            .map(|(i, s)| (s.chars().rev().collect(), i))
            .collect();
        let is_palindrome = |s: &String, left: usize, right: usize| -> bool {
            let bs = s.as_bytes();
            let len = if left > right { 0 } else { right - left + 1 };
            for i in 0..len / 2 {
                if bs[left + i] != bs[right - i] {
                    return false;
                }
            }
            true
        };
        let mut ans = Vec::new();
        for (i, word) in words.iter().enumerate() {
            if word.is_empty() {
                continue;
            }
            let m = word.len();
            for j in 0..=m {
                if is_palindrome(word, j, m - 1) {
                    let rs = if j>0{(word[..=j - 1]).to_string()}else{String::new()};
                    if let Some(&k) = w2i.get(&rs) {
                        if k != i {
                            ans.push(vec![i as i32, k as i32]);
                        }
                    }
                }
                if j > 0 && is_palindrome(word, 0, j - 1) {
                    if let Some(&k) = w2i.get(&(word[j..=m - 1]).to_string()) {
                        if k != i {
                            ans.push(vec![k as i32, i as i32]);
                        }
                    }
                }
            }
        }
        ans
    }
}
// @lc code=end
