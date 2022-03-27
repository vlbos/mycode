/*
 * @lc app=leetcode id=1785 lang=rust
 *
 * [1785] Minimum Elements to Add to Form a Given Sum
 */

// @lc code=start
impl Solution {
    pub fn min_elements(nums: Vec<i32>, limit: i32, goal: i32) -> i32 {
        let sum = nums.iter().map(|x| (*x) as i64).sum::<i64>();
        let d = (sum - goal as i64).abs();
        let mut ans = d / limit as i64;
        if d % limit as i64 > 0 {
            ans += 1;
        };
        ans as _
    }
}
// @lc code=end
