/*
 * @lc app=leetcode.cn id=682 lang=rust
 *
 * [682] 棒球比赛
 */

// @lc code=start
impl Solution {
    pub fn cal_points(ops: Vec<String>) -> i32 {
        let mut r = Vec::<i32>::new();
        for op in &ops {
            if op == "C" {
                r.pop();
            } else if op == "D" {
                r.push(r[r.len() - 1] * 2);
            } else if op == "+" {
                r.push(r[r.len() - 1] + r[r.len() - 2]);
            } else {
                r.push(op.parse::<i32>().unwrap());
            }
        }
        r.iter().sum()
    }
}
// @lc code=end
