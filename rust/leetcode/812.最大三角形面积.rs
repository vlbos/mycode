/*
 * @lc app=leetcode.cn id=812 lang=rust
 *
 * [812] 最大三角形面积
 */

// @lc code=start
impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut m = 0f64;
        let area = |points: Vec<Vec<i32>>| -> f64 {
            let mut ma = 0f64;
            for i in 0..points.len() {
                ma += ((points[i][0] + points[(i + 1) % points.len()][0])
                    * (points[(i + 1) % points.len()][1] - points[i][1]))
                    as f64
            }
            ma.abs() / 2f64
        };

        for i in 0..points.len() - 2 {
            for j in 1..points.len() - 1 {
                for k in 2..points.len() {
                    let a = area(
                        vec![points[i].clone(), points[j].clone(), points[k].clone()].to_vec(),
                    );
                    if a > m {
                        m = a;
                    }
                }
            }
        }
        m
    }
}
// @lc code=end
