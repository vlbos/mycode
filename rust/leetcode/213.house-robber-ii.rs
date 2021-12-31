/*
 * @lc app=leetcode id=213 lang=rust
 *
 * [213] House Robber II
 */

// @lc code=start
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let rob_range=|nums: &Vec<i32>,i:usize,j:usize|->i32{
            let mut first = nums[i];
            let mut second = nums[i].max(nums[i+1]);
            for k in i+2..=j{
                let t = second;
                second = second.max(nums[k]+first);
                first = t;
            }
            second
        };
        if nums.len()<2{
            return nums[0];
        }else if nums.len()<3{
            return nums[0].max(nums[1])
        }
        rob_range(&nums,0,nums.len()-2).max(rob_range(&nums,1,nums.len()-1))
    }
}
// @lc code=end

2