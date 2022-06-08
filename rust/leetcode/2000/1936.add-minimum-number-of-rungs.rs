/*
 * @lc app=leetcode id=1936 lang=rust
 *
 * [1936] Add Minimum Number of Rungs
 */

// @lc code=start
impl Solution {
    pub fn add_rungs(rungs: Vec<i32>, dist: i32) -> i32 {
        let mut ans = 0;
        let mut cur = 0;
        for &h in &rungs {
            let d = h - cur;
            ans += (d - 1) / dist;
            cur = h;
        }
        ans
    }
}
// @lc code=end
