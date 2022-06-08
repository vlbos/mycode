/*
 * @lc app=leetcode id=413 lang=rust
 *
 * [413] Arithmetic Slices
 */

// @lc code=start
impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        if nums.len()==1{
        return 0;
        }
        let mut d = nums[0]-nums[1];
        let mut t = 0;
        let mut ans = 0;
        for i in 2..nums.len(){
            if nums[i-1]-nums[i]==d{
                t+=1;
            }else{
                d=nums[i-1]-nums[i];
                t=0;
            }
            ans+=t;
        }
        ans
    }
}
// @lc code=end

