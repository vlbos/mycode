/*
 * @lc app=leetcode id=1619 lang=rust
 *
 * [1619] Mean of Array After Removing Some Elements
 */

// @lc code=start
impl Solution {
    pub fn trim_mean(arr: Vec<i32>) -> f64 {
        let mut a = arr;
        a.sort();
        let r = a.len()/20;
        let aa = &a[r..a.len()-r];
        aa.iter().sum::<i32>() as f64/aa.len() as f64
    }
}
// @lc code=end

