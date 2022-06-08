/*
 * @lc app=leetcode id=446 lang=rust
 *
 * [446] Arithmetic Slices II - Subsequence
 */

// @lc code=start
impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let n = nums.len();
        let mut f = vec![std::collections::HashMap::new(); n];
        for (i, &num) in nums.iter().enumerate() {
            for j in 0..i {
                let d = num as i64 - nums[j] as i64;
                let cnt = *f[j].get(&d).unwrap_or(&0);
                ans += cnt;
                *f[i].entry(d).or_insert(0) += cnt + 1;
            }
        }
        ans
    }
}
// @lc code=end
