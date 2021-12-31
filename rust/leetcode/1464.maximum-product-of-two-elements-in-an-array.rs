/*
 * @lc app=leetcode id=1464 lang=rust
 *
 * [1464] Maximum Product of Two Elements in an Array
 */

// @lc code=start
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        if nums.len()==2{
            return (nums[0]-1)*(nums[1]-1);
        }
        let mut n =nums;
        n.sort();
        (n[n.len()-1]-1)*(n[n.len()-2]-1)
    }
}
// @lc code=end

