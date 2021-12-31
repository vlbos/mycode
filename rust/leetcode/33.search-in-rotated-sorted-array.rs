/*
 * @lc app=leetcode id=33 lang=rust
 *
 * [33] Search in Rotated Sorted Array
 */

// @lc code=start
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if let Some(i)=nums.iter().position(|x|*x==target){
        i as i32} else{-1}
    }
}
// @lc code=end

