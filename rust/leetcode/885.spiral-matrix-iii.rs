/*
 * @lc app=leetcode id=885 lang=rust
 *
 * [885] Spiral Matrix III
 */

// @lc code=start
impl Solution {
    pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        ans.push(vec![r_start, c_start]);
        if rows == 1 && cols == 1 {
            return ans;
        }
        let mut r_start = r_start;
        let mut c_start = c_start;
        let d = vec![vec![0, 1], vec![1, 0], vec![0, -1], vec![-1, 0]];
        for k in (1..2 * (rows + cols)).step_by(2) {
            for i in 0..4 {
                let dk = k + i / 2;
                for _ in 0..dk {
                    r_start += d[i as usize][0];
                    c_start += d[i as usize][1];
                    if r_start >= 0 && r_start < rows && c_start >= 0 && c_start < cols {
                        ans.push(vec![r_start, c_start]);
                        if ans.len() as i32 == rows * cols {
                            return ans;
                        }
                    }
                }
            }
        }
        ans
    }
}
// @lc code=end
