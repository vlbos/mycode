/*
 * @lc app=leetcode id=910 lang=rust
 *
 * [910] Smallest Range II
 */

// @lc code=start
impl Solution {
    pub fn smallest_range_ii(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums=nums;
        nums.sort();
        let last = nums.last().unwrap();
        let first = nums.first().unwrap();
        let mut ans = last-first;
        for i in 0..nums.len()-1{
            let a = nums[i];
            let b = nums[i+1];
            let high = (last-k).max(a+k);
            let low = (first+k).min(b-k);
            ans = ans.min(high-low);
        }
        ans
    }
}
// @lc code=end

