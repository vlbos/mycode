/*
 * @lc app=leetcode id=1200 lang=rust
 *
 * [1200] Minimum Absolute Difference
 */

// @lc code=start
impl Solution {
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut a = arr;
        a.sort();
        let mut r = Vec::new();
        let mut m = i32::MAX;
        for i in 0..a.len() - 1 {
            let d = a[i + 1] - a[i];
            if d < m {
                r.clear();
                r.push(vec![a[i], a[i + 1]].to_vec());
                m = d;
            } else if d == m {
                r.push(vec![a[i], a[i + 1]].to_vec());
            }
        }
        r
    }
}
// @lc code=end
