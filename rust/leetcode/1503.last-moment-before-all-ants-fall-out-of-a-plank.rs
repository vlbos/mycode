/*
 * @lc app=leetcode id=1503 lang=rust
 *
 * [1503] Last Moment Before All Ants Fall Out of a Plank
 */

// @lc code=start
impl Solution {
    pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
        let mut ans = 0;
        for &l in &left {
            ans = ans.max(l);
        }
        for &r in &right {
            ans = ans.max(n - r);
        }
        ans
    }
}
// @lc code=end
