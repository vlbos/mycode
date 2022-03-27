/*
 * @lc app=leetcode id=1186 lang=rust
 *
 * [1186] Maximum Subarray Sum with One Deletion
 */

// @lc code=start
impl Solution {
    pub fn maximum_sum(arr: Vec<i32>) -> i32 {
        let inf= i32::MIN/2;
        let (mut s1,mut s2,mut max)=(inf,inf,inf);
        for &a in &arr{
            s2 = s1.max(s2+a);
            s1= a.max(s1+a);
            max = max.max(s1).max(s2);
        }
        max
    }
}
// @lc code=end

