/*
 * @lc app=leetcode id=1887 lang=rust
 *
 * [1887] Reduction Operations to Make the Array Elements Equal
 */

// @lc code=start
impl Solution {
    pub fn reduction_operations(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        let mut ans = 0;
        let mut cnt = 0;
        for i in 1..n {
            if nums[i] != nums[i - 1] {
                cnt += 1;
            }
            ans += cnt;
        }
        ans
    }
}
// @lc code=end
