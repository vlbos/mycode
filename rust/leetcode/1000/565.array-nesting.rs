/*
 * @lc app=leetcode id=565 lang=rust
 *
 * [565] Array Nesting
 */

// @lc code=start
impl Solution {
    pub fn array_nesting(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut ans = 0;
        let nn = nums.len() as i32;
        for i in 0..nums.len(){
            let mut s = nums[i] as usize;
            let mut count = 0;
            while s<nums.len() &&  nums[s]!=nn{
                 let t = s;
                 count+=1;
                s=nums[s] as usize;
                nums[t]=nn;
            }
            ans = ans.max(count);
        }
        ans
    }
}
// @lc code=end

