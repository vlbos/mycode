/*
 * @lc app=leetcode id=1030 lang=rust
 *
 * [1030] Matrix Cells in Distance Order
 */

// @lc code=start
impl Solution {
    pub fn all_cells_dist_order(
        rows: i32,
        cols: i32,
        r_center: i32,
        c_center: i32,
    ) -> Vec<Vec<i32>> {
        let mut a = Vec::<Vec::<i32>>::new();
        for r in 0..rows {
            for c in 0..cols {
                let d = (r_center - r).abs() + (c_center - c).abs();
                let mut i = a.len();
                for j in (0..i).rev() {
                    let d1 = (r_center - a[j][0]).abs() + (c_center - a[j][1]).abs();
                    if d < d1 {
                        if i > 0 {
                            i -= 1;
                        } else {
                            break;
                        }
                    }
                }
                a.insert(i,vec![r,c].to_vec());
            }
        }
        a
    }
}
// @lc code=end
