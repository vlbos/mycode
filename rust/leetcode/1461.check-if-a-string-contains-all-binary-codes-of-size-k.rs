/*
 * @lc app=leetcode id=1461 lang=rust
 *
 * [1461] Check If a String Contains All Binary Codes of Size K
 */

// @lc code=start
impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let k = k as usize;
        if s.len() < (1 << k) + k - 1 {
            return false;
        }
        let mut e = std::collections::HashSet::new();
        let mut num = 0;
        let bs = s.as_bytes();
        for i in 0..k {
            num <<= 1;
            num += (bs[i] - b'0') as i32;
        }
        e.insert(num);
        for i in 1..=bs.len() - k {
            num = (num - (((bs[i - 1] - b'0') as i32) << (k - 1))) * 2 + (bs[i + k - 1]-b'0') as i32;
            e.insert(num);
            
        }
        e.len() == (1 << k)
    }
}
// @lc code=end
