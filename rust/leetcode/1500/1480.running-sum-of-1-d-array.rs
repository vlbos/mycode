/*
 * @lc app=leetcode id=1480 lang=rust
 *
 * [1480] Running Sum of 1d Array
 */

// @lc code=start
impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut sum =0;
        let mut ans = Vec::new();
        for n in &nums{
            sum+=*n;
            ans.push(sum);
        }
        ans
    }
}
// @lc code=end

