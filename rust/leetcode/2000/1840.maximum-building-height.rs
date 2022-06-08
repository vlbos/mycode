/*
 * @lc app=leetcode id=1840 lang=rust
 *
 * [1840] Maximum Building Height
 */

// @lc code=start
impl Solution {
    pub fn max_building(n: i32, restrictions: Vec<Vec<i32>>) -> i32 {
        let mut r = restrictions;
        r.push(vec![1, 0]);
        r.sort();
        if r[r.len() - 1][0] != n {
            r.push(vec![n, n - 1]);
        }
        let m = r.len();
        for i in 1..m {
            r[i][1] = r[i][1].min(r[i - 1][1] + r[i][0] - r[i - 1][0]);
        }
        for i in (0..m - 1).rev() {
            r[i][1] = r[i][1].min(r[i + 1][1] + r[i + 1][0] - r[i][0]);
        }
        let mut ans = 0;
        for i in 0..m - 1 {
            ans = ans.max((r[i + 1][1] + r[i][1] + r[i + 1][0] - r[i][0]) / 2);
        }
        ans
    }
}
// @lc code=end
