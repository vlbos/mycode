/*
 * @lc app=leetcode id=1515 lang=rust
 *
 * [1515] Best Position for a Service Centre
 */

// @lc code=start
impl Solution {
    pub fn get_min_dist_sum(positions: Vec<Vec<i32>>) -> f64 {
        let eps = 1e-7;
        let get_dist = |xc: f64, yc: f64| {
            let mut ans = 0.0;
            for pos in &positions {
                let (dx, dy) = (pos[0] as f64 - xc , pos[1]  as f64- yc);
                ans += (dx * dx + dy * dy).sqrt();
            }
            ans
        };
        let check_optimal = |xc: f64| {
            let (mut yleft, mut yright) = (0.0, 100.0);
            while yright - yleft > eps {
                let (yfirst, ysecond) =
                    ((yleft + yleft + yright) / 3.0, (yleft + yright + yright) / 3.0);
                if get_dist(xc, yfirst) < get_dist(xc, ysecond) {
                    yright = ysecond;
                } else {
                    yleft = yfirst;
                }
            }
            get_dist(xc, yleft)
        };
        let (mut yleft, mut yright) = (0.0, 100.0);
        while yright - yleft > eps {
            let (yfirst, ysecond) = ((yleft + yleft + yright) / 3.0, (yleft + yright + yright) / 3.0);
            if check_optimal(yfirst) < check_optimal(ysecond) {
                yright = ysecond;
            } else {
                yleft = yfirst;
            }
        }
        check_optimal(yleft)
    }
}
// @lc code=end
