/*
 * @lc app=leetcode id=1402 lang=rust
 *
 * [1402] Reducing Dishes
 */

// @lc code=start
impl Solution {
    pub fn max_satisfaction(satisfaction: Vec<i32>) -> i32 {
        let mut satisfaction = satisfaction;
        satisfaction.sort_by_key(|x| -x);
        let mut presum = 0;
        let mut ans = 0;
        for &si in &satisfaction {
            if presum + si > 0 {
                presum += si;
                ans += presum;
            } else {
                break;
            }
        }
        ans
    }
}
// @lc code=end
