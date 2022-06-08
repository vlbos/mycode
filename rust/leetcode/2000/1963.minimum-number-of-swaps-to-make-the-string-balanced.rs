/*
 * @lc app=leetcode id=1963 lang=rust
 *
 * [1963] Minimum Number of Swaps to Make the String Balanced
 */

// @lc code=start
impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut cnt = 0;
        let mut min = 0;
        for c in s.chars() {
            if c == '[' {
                cnt += 1;
            } else {
                cnt -= 1;
                min = min.min(cnt);
            }
        }
        (-min + 1) / 2
    }
}
// @lc code=end
