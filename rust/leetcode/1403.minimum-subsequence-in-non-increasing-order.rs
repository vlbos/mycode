/*
 * @lc app=leetcode id=1403 lang=rust
 *
 * [1403] Minimum Subsequence in Non-Increasing Order
 */

// @lc code=start
impl Solution {
    pub fn min_subsequence(nums: Vec<i32>) -> Vec<i32> {
        let mut sum :i32= nums.iter().sum();
        let mut n = nums;
        n.sort();
        let mut s = 0;
        let mut ans = Vec::new();
        for vv in n.iter().rev() {
            let v = *vv;
            if s + v <= sum - v {
                s += v;
                sum -= v;
                ans.push(v);
            } else {
                ans.push(v);
                break;
            }
        }
        ans
    }
}
// @lc code=end
