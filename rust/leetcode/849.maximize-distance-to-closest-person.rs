/*
 * @lc app=leetcode id=849 lang=rust
 *
 * [849] Maximize Distance to Closest Person
 */

// @lc code=start
impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let n = seats.len();
        let mut ans = 0;
        let mut e = 0;
        for v in &seats {
            if *v == 1 {
                e = 0;
            } else {
                e += 1;
                ans = ans.max((e + 1) / 2);
            }
        }
        for i in 0..n {
            if seats[i] == 1 {
                ans = ans.max(i);
                break;
            }
        }
        for i in (0..n).rev() {
            if seats[i] == 1 {
                ans = ans.max(n - i - 1);
                break;
            }
        }
        ans as i32
    }
}
// @lc code=end
