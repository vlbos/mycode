/*
 * @lc app=leetcode id=442 lang=rust
 *
 * [442] Find All Duplicates in an Array
 */

// @lc code=start
impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let n = nums.len() as i32;
        for i in 0..nums.len(){
            let j = nums[i] as usize;
            nums[(j-1)%n as usize]+=n;
        }
        let mut ans = Vec::new();
        for (i,v) in nums.iter().enumerate(){
            if *v>2*n {
                ans.push(i as i32+1);
            }
        }
        ans
    }
}
// @lc code=end

