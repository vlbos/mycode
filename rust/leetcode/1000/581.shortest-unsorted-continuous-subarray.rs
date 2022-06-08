/*
 * @lc app=leetcode id=581 lang=rust
 *
 * [581] Shortest Unsorted Continuous Subarray
 */

// @lc code=start
impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let n =nums.len();
        let mut l = n;
        let mut r = n;
        let mut max = i32::MIN;
        let mut min = i32::MAX;
        for  i in 0..n{
                if nums[i]>=max{
                    max=nums[i];
                }else{
                    r=i;
                }
                let j = n-i-1;
                if nums[j]<=min{
                    min=nums[j];
                }else{
                    l=j;
                }
        }
        if r==n {0}else{(r-l+1) as i32}
    }
}
// @lc code=end

