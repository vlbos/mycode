/*
 * @lc app=leetcode id=1389 lang=rust
 *
 * [1389] Create Target Array in the Given Order
 */

// @lc code=start
impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();
        for (i,n) in nums.iter().enumerate(){
            ans.insert(index[i] as usize,*n);
        }
        ans
    }
}
// @lc code=end

