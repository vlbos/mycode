/*
 * @lc app=leetcode id=1827 lang=rust
 *
 * [1827] Minimum Operations to Make the Array Increasing
 */

// @lc code=start
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut n = nums;
        let mut s = 0;
        for i in 1..n.len(){
            if n[i]<=n[i-1]{
                let d   = n[i-1]+1-n[i];
                n[i]=n[i-1]+1;
                s+=d;
            }
        }
        s
    }
}
// @lc code=end

