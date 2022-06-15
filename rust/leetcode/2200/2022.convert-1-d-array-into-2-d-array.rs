/*
 * @lc app=leetcode id=2022 lang=rust
 *
 * [2022] Convert 1D Array Into 2D Array
 */

// @lc code=start
impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        let (m, n) = (m as usize, n as usize);
        if m * n != original.len() {
            return Vec::new();
        }
        let mut ans = Vec::new();
        for i in 0..m {
            ans.push(original[n * i..n * (i + 1)].to_vec());
        }
        ans
    }
}
// @lc code=end
