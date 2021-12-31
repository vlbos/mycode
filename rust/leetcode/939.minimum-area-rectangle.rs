/*
 * @lc app=leetcode id=939 lang=rust
 *
 * [939] Minimum Area Rectangle
 */

// @lc code=start
impl Solution {
    pub fn min_area_rect(points: Vec<Vec<i32>>) -> i32 {
        let p = points.iter().collect::<std::collections::HashSet<_>>();
        let n = points.len();
        let mut ans = i32::MAX;
        for i in 0..n {
            for j in i + 1..n {
                if points[i][0] != points[j][0] && points[i][1] != points[j][1] {
                    if p.contains(&vec![points[i][0], points[j][1]])
                        && p.contains(&vec![points[j][0], points[i][1]])
                    {
                        ans = ans.min(
                            (points[i][0] - points[j][0]).abs()
                                * (points[i][1] - points[j][1]).abs(),
                        );
                    }
                }
            }
        }
        if ans == i32::MAX {
            0
        } else {
            ans
        }
    }
}
// @lc code=end
