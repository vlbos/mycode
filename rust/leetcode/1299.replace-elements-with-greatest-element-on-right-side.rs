/*
 * @lc app=leetcode id=1299 lang=rust
 *
 * [1299] Replace Elements with Greatest Element on Right Side
 */

// @lc code=start
impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut a = vec![0; arr.len()];
        a[arr.len() - 1] = -1;
        for j in (0..arr.len() - 1).rev() {
            a[j] = a[j + 1].max(arr[j + 1]);
        }
        a
    }
}
// @lc code=end
