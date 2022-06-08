/*
 * @lc app=leetcode id=452 lang=rust
 *
 * [452] Minimum Number of Arrows to Burst Balloons
 */

// @lc code=start
impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut points=points;
        points.sort_by(|a,b|a[1].cmp(&b[1]));
        let mut pos = points[0][1];
        let mut ans = 1;
        for i in 0..points.len(){
            if points[i][0]>pos{
                    ans+=1;
                pos=points[i][1];
            }
        }
        ans
    }
}
// @lc code=end

