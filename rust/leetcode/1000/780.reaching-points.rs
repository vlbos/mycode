/*
 * @lc app=leetcode id=780 lang=rust
 *
 * [780] Reaching Points
 */

// @lc code=start
impl Solution {
    pub fn reaching_points(sx: i32, sy: i32, tx: i32, ty: i32) -> bool {
        let (mut tx, mut ty) = (tx, ty);
        while tx > sx && ty > sy && tx != ty {
            if tx > ty {
                tx %= ty;
            } else {
                ty %= tx;
            }
        }
        if tx == sx && ty == sy {
            return true;
        }
        if tx == sx {
            return ty > sy && (ty - sy) % tx == 0;
        }
        if ty == sy {
            return tx > sx && (tx - sx) % ty == 0;
        }
        false
    }
}
// @lc code=end
