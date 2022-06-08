/*
 * @lc app=leetcode id=1630 lang=rust
 *
 * [1630] Arithmetic Subarrays
 */

// @lc code=start
impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
       let check = |nums: &Vec<i32>| -> bool {
            let t = nums[1] - nums[0];
            for i in 1..nums.len() {
                if nums[i] - nums[i - 1] != t {
                    return false;
                }
            }
            true
        };
        let mut ans = Vec::new();
        for (i, &s) in l.iter().enumerate() {
            let mut nn = (&nums[s as usize..=r[i] as usize]).to_vec();
            nn.sort();
            ans.push(check(&nn));
        }
        ans
    }
}
// @lc code=end
