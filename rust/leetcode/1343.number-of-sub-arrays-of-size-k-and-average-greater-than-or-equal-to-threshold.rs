/*
 * @lc app=leetcode id=1343 lang=rust
 *
 * [1343] Number of Sub-arrays of Size K and Average Greater than or Equal to Threshold
 */

// @lc code=start
impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let sum: i32 = arr[..k as usize].iter().sum();
        let sumt = k * threshold;
        let mut addr = sum - sumt;
        let mut ans = 0;
        if addr >= 0 {
            ans += 1;
        }
        let mut pos = k as usize;
        for i in 0..arr.len() - k as usize {
            addr = addr + arr[pos] - arr[i];
            if addr >= 0 {
                ans += 1;
            }
            pos += 1;
        }
        ans
    }
}
// @lc code=end
