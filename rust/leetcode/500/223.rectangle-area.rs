/*
 * @lc app=leetcode id=223 lang=rust
 *
 * [223] Rectangle Area
 */

// @lc code=start
impl Solution {
    pub fn compute_area(
        ax1: i32,
        ay1: i32,
        ax2: i32,
        ay2: i32,
        bx1: i32,
        by1: i32,
        bx2: i32,
        by2: i32,
    ) -> i32 {
        let areaa = (ax2 - ax1) * (ay2 - ay1);
        let areab = (bx2 - bx1) * (by2 - by1);
        let dupx = ax2.min(bx2)-ax1.max(bx1);
        let dupy = ay2.min(by2)-ay1.max(by1);
        areaa + areab - dupx.max(0) * dupy.max(0)
    }
}
// @lc code=end
