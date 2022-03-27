/*
 * @lc app=leetcode id=1959 lang=rust
 *
 * [1959] Minimum Total Space Wasted With K Resizing Operations
 */

// @lc code=start
impl Solution {
    pub fn min_space_wasted_k_resizing(nums: Vec<i32>, k: i32) -> i32 {
       let k2 = k as usize+2;
        let n = nums.len();
        let mut g = vec![vec![0; n]; n];
        for i in 0..n {
            let mut best = i32::MIN;
            let mut total = 0;
            for j in i..n {
                best = best.max(nums[j]);
                total += nums[j];
                g[i][j] = best * ((j - i + 1) as i32) - total;
            }
        }
        let mut f = vec![vec![i32::MAX / 2; k2]; n];
        for i in 0..n {
            for j in 1..k2{
                for i0 in 0..=i {
                    f[i][j] = f[i][j].min(if i0 == 0 {
                        0
                    } else {
                        f[i0 - 1][j - 1] 
                    }+ g[i0][i]);
                }
            }
        }
        f[n - 1][k2 - 1]
    }
}
// @lc code=end
