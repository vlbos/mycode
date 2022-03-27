/*
 * @lc app=leetcode id=1997 lang=rust
 *
 * [1997] First Day Where You Have Been in All the Rooms
 */

// @lc code=start
impl Solution {
    pub fn first_day_been_in_all_rooms(next_visit: Vec<i32>) -> i32 {
        let n = next_visit.len();
        let mut f = vec![0i64; n];
        for i in 1..n {
            f[i] = (f[i - 1] + 1 + f[i - 1] - f[next_visit[i - 1] as usize] + 1 + 1000_000_007)
                % 1000_000_007;
        }
       ( f[n - 1] % 1000_000_007) as _
    }
}
// @lc code=end
