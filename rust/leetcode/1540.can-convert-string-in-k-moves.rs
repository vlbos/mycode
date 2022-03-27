/*
 * @lc app=leetcode id=1540 lang=rust
 *
 * [1540] Can Convert String in K Moves
 */

// @lc code=start
impl Solution {
    pub fn can_convert_string(s: String, t: String, k: i32) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let e = 26;
        let mut counts = vec![0; e];
        let tb = t.as_bytes();
        for (i, b) in s.bytes().enumerate() {
            let mut d = if tb[i] < b { 26 - b + tb[i] } else { tb[i] - b };
            counts[d as usize] += 1;
        }
        for i in 1..e {
            if counts[i] == 0 {
                continue;
            }
            let mv = i + (counts[i] - 1) * e;
            if mv > k as usize {
                return false;
            }
        }
        true
    }
}
// @lc code=end
