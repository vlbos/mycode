/*
 * @lc app=leetcode id=1703 lang=rust
 *
 * [1703] Minimum Adjacent Swaps for K Consecutive Ones
 */

// @lc code=start
impl Solution {
    pub fn min_moves(nums: Vec<i32>, k: i32) -> i32 {
        if k == 1 {
            return 0;
        }
        let mut g = Vec::new();
        let mut sum = vec![0];
        let mut count = -1;
        for (i, &v) in nums.iter().enumerate() {
            if v == 1 {
                count += 1;
                let gl = i as i32 - count;
                g.push(gl);
                let last = sum[sum.len() - 1];
                sum.push(last + gl);
            }
        }
        let mut ans = i32::MAX;
        let ki = k as usize;
        for ii in 0..=g.len() - ki {
            let i = ii as i32;
            let mid = (i + i + k - 1) / 2;
            let midi = mid as usize;
            let q = g[midi];
            ans = ans.min(
                (2 * (mid - i) - k + 1) * q + (sum[ii + ki] - sum[midi + 1]) - (sum[midi] - sum[ii])
            );
        }
        ans
    }
}
// @lc code=end
