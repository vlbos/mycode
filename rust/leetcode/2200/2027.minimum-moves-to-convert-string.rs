/*
 * @lc app=leetcode id=2027 lang=rust
 *
 * [2027] Minimum Moves to Convert String
 */

// @lc code=start
impl Solution {
    pub fn minimum_moves(s: String) -> i32 {
        let bs = s.as_bytes();
        let mut ans = 0;
        let mut i = 0;
        while i < bs.len() {
            if bs[i] == b'X' {
                ans += 1;
                i += 2;
            }
            i+=1;
        }
        ans
    }
}
// @lc code=end
