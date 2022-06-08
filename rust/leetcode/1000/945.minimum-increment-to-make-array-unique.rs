/*
 * @lc app=leetcode id=945 lang=rust
 *
 * [945] Minimum Increment to Make Array Unique
 */

// @lc code=start
impl Solution {
    pub fn min_increment_for_unique(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        let mut ans = 0;
        let mut taken = 0;
        for i in 1..n{
            if nums[i-1]==nums[i]{
                taken +=1;
                ans-=nums[i];
            }else{
                let give = taken.min(nums[i]-nums[i-1]-1);
                ans+=give*(give+1)/2+give*nums[i-1];
                taken-=give;
            }
        }
        if !nums.is_empty(){
               ans+=taken*(taken+1)/2+taken*nums[n-1];
        }
        ans
    }
}
// @lc code=end

