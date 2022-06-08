/*
 * @lc app=leetcode id=2090 lang=rust
 *
 * [2090] K Radius Subarray Averages
 */

// @lc code=start
impl Solution {
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let n = nums.len();
        let k2 = 2 * k + 1;
        if n < k2 {
            return vec![-1; n];
        }
        let mut ans = vec![-1; k];
        let mut sum = nums[..k2].iter().map(|x|(*x) as i64).sum::<i64>();
        for i in k..n - k {
            ans.push((sum/k2 as i64) as i32);
            if i+k+1<n{
                sum += (nums[i + k + 1] - nums[i - k]) as i64;
            }
        }
        ans.extend(vec![-1; k]);
        ans
    }
}
// @lc code=end
