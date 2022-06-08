/*
 * @lc app=leetcode id=1695 lang=rust
 *
 * [1695] Maximum Erasure Value
 */

// @lc code=start
impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut i= 0;
        let mut ans = 0;
        let mut s = std::collections::HashSet::new();
        for &v in &nums {
            while s.contains(&v){
                s.remove(&nums[i]);
                sum -= nums[i];
                i += 1;
            }
            sum += v;
            s.insert(v);
            ans = ans.max(sum);
        }
        ans
    }
}
// @lc code=end
