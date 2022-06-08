/*
 * @lc app=leetcode id=899 lang=rust
 *
 * [899] Orderly Queue
 */

// @lc code=start
impl Solution {
    pub fn orderly_queue(s: String, k: i32) -> String {
        if k == 1 {
            let i = (0..s.len()).min_by_key(|i| s[*i..].to_string() + &s[0..*i]).unwrap();
            return s[i..].to_string() + &s[0..i];
        }
        let mut sc: Vec<char> = s.chars().collect();
        sc.sort();
        sc.iter().collect()
    }
}
// @lc code=end
