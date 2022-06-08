/*
 * @lc app=leetcode id=1453 lang=rust
 *
 * [1453] Maximum Number of Darts Inside of a Circular Dartboard
 */

// @lc code=start
impl Solution {
    pub fn num_points(darts: Vec<Vec<i32>>, r: i32) -> i32 {
         let n = darts.len();
        let mut ans = 1;
        let rr = r as f64;
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }
                let (dx, dy) = (darts[i][0] - darts[j][0], darts[i][1] - darts[j][1]);
                if dx * dx + dy * dy > 4 * r * r {
                    continue;
                }
                let (dx, dy) = (dx as f64, dy as f64);
                let dis = (dx * dx + dy * dy).sqrt();
                let (cs, sn) = (dx / dis, dy / dis);

                let ca = dis.abs() / (2.0 * rr);
                let sa = (1.0 - ca * ca).sqrt();
                let (center_x, center_y) = (
                    darts[j][0] as f64 + rr * (cs * ca - sn * sa),
                    darts[j][1] as f64 + rr * (cs * sa + sn * ca),
                );
                let mut nn = 0;
                for k in 0..n {
                    let (dx, dy) = (center_x - darts[k][0] as f64, center_y  - darts[k][1] as f64);
                    if dx * dx + dy * dy < rr * rr+0.0001 {
                        nn += 1;
                    }
                }
                ans = ans.max(nn);
            }
        }
        ans
    }
}
// @lc code=end
