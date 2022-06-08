/*
 * @lc app=leetcode id=1701 lang=rust
 *
 * [1701] Average Waiting Time
 */

// @lc code=start
impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let mut s = customers[0][0] as i64;
        let mut ans = 0;
        for c in &customers {
            let ss = s.max(c[0] as i64);
            ans += ss + c[1]  as i64 - c[0]  as i64;
            s =ss+c[1]  as i64;
        }
        ans as f64 / customers.len() as f64
    }
}
// @lc code=end
