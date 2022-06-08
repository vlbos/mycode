/*
 * @lc app=leetcode id=973 lang=rust
 *
 * [973] K Closest Points to Origin
 */

// @lc code=start
impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
         let mut p= points;
           p.sort_by(|a,b|(a[0]*a[0]+a[1]*a[1]).cmp(&(b[0]*b[0]+b[1]*b[1])));
           p[..k as usize].to_vec()
    }
}
// @lc code=end

