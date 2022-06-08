/*
 * @lc app=leetcode id=2012 lang=rust
 *
 * [2012] Sum of Beauty in the Array
 */

// @lc code=start
impl Solution {
    pub fn sum_of_beauties(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut suf_min = vec![0; n];
        suf_min[n - 1] = nums[n - 1];
        for i in (1..n - 1).rev() {
            suf_min[i] = suf_min[i + 1].min(nums[i]);
        }
        let mut pre_max = nums[0];
        let mut ans = 0;
        for i in 1..n - 1 {
            let v = nums[i];
            if pre_max < v && v < suf_min[i + 1] {
                ans += 2;
            } else if nums[i - 1] < v && v < nums[i + 1] {
                ans += 1;
            }
            pre_max = pre_max.max(v);
        }
        ans
    }
}
// @lc code=end
