/*
 * @lc app=leetcode id=462 lang=rust
 *
 * [462] Minimum Moves to Equal Array Elements II
 */

// @lc code=start
impl Solution {
    pub fn min_moves2(nums: Vec<i32>) -> i32 {
        let mut nums=nums;
        nums.sort();
        let m= nums[nums.len()/2];
        let mut ans = 0;
        for v in &nums{
            ans+= (*v-m).abs();
        }
        ans
    }
}
// @lc code=end

