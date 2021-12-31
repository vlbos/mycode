/*
 * @lc app=leetcode id=1266 lang=rust
 *
 * [1266] Minimum Time Visiting All Points
 */

// @lc code=start
impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut ans=0;
        for i in 1..points.len(){
             ans+= (points[i][0]-points[i-1][0]).abs().max((points[i][1]-points[i-1][1]).abs());
        }
        ans
    }
}
// @lc code=end

