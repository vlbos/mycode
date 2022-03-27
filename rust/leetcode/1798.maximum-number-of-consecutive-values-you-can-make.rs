/*
 * @lc app=leetcode id=1798 lang=rust
 *
 * [1798] Maximum Number of Consecutive Values You Can Make
 */

// @lc code=start
impl Solution {
    pub fn get_maximum_consecutive(coins: Vec<i32>) -> i32 {
        let mut coins = coins;
        coins.sort();
        let mut ans = 0;
        for &c in &coins {
            if c > ans + 1 {
                break;
            }
            ans += c;
        }
        ans + 1
    }
}
// @lc code=end
