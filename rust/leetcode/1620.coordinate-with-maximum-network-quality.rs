/*
 * @lc app=leetcode id=1620 lang=rust
 *
 * [1620] Coordinate With Maximum Network Quality
 */

// @lc code=start
impl Solution {
    pub fn best_coordinate(towers: Vec<Vec<i32>>, radius: i32) -> Vec<i32> {
        let n = towers.len();
        let mut max = 0;
        let mut ans = vec![0,0];
        let r2 = radius * radius;
        let (mut minx, mut miny, mut maxx, mut maxy) = (i32::MAX, i32::MAX, 0, 0);
        for t in &towers {
            minx = minx.min(t[0]);
            maxx = maxx.max(t[0]);
            miny = miny.min(t[1]);
            maxy = maxy.max(t[1]);
        }
        for i in minx..=maxx {
            for j in miny..=maxy {
                let mut q = 0;
                for t in &towers {
                    let (dx, dy) = (i - t[0], j - t[1]);
                    let d = dx * dx + dy * dy;
                    if d <= r2 {
                        q += (t[2] as f64 / (1.0 + (d as f64).sqrt())) as i32;
                    }
                }
                if q > max {
                    max = q;
                    ans = vec![i, j];
                }
            }
        }
        ans
    }
}
// @lc code=end
