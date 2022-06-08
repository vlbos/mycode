/*
 * @lc app=leetcode id=1550 lang=rust
 *
 * [1550] Three Consecutive Odds
 */

// @lc code=start
impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut cnt = 0;
        for a in &arr {
            if a % 2 != 0 {
                cnt += 1;
                if  cnt >= 3{
                    return true;
                }
            } else {
                cnt = 0;
            }
        }
        false
    }
}
// @lc code=end
