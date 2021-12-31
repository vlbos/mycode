/*
 * @lc app=leetcode id=532 lang=rust
 *
 * [532] K-diff Pairs in an Array
 */

// @lc code=start
impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut l = 0;
        let mut r = 1;
        let mut ans = 0;
        while r<nums.len(){
            if nums[r]==nums[r-1] && r-1>l && nums[r-1]-nums[l]==k{
                r+=1;
                continue;
            }
            if nums[r]-nums[l]==k{
                ans+=1;
                r+=1;
            }else if nums[r]-nums[l]>k && l+1<r{
                l+=1;
            }else{
                r+=1;
            }
        }
       ans
    }
}
// @lc code=end

