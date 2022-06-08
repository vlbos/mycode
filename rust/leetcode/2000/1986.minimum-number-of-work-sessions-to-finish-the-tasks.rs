/*
 * @lc app=leetcode id=1986 lang=rust
 *
 * [1986] Minimum Number of Work Sessions to Finish the Tasks
 */

// @lc code=start
impl Solution {
    pub fn min_sessions(tasks: Vec<i32>, session_time: i32) -> i32 {
         let n = tasks.len();
        let n2 = 1 << n;
        let mut f = vec![(i32::MAX, i32::MAX); n2];
        f[0] = (1, 0);
        let add = |o: &(i32, i32), x: i32| -> (i32, i32) {
            if o.1 + x <= session_time {
                (o.0, o.1 + x)
            } else {
                (o.0 + 1, x)
            }
        };
        for mask in 1..n2 {
            for i in 0..n {
                if (mask & (1 << i)) > 0 {
                    f[mask] = f[mask].min(add(&f[(mask ^ (1 << i)) as usize], tasks[i]));
                }
            }
        }
        f[n2 - 1].0
    }
}
// @lc code=end
