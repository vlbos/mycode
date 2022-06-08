/*
 * @lc app=leetcode id=1478 lang=rust
 *
 * [1478] Allocate Mailboxes
 */

// @lc code=start
impl Solution {
    pub fn min_distance(houses: Vec<i32>, k: i32) -> i32 {
        let (n, k) = (houses.len(), k as usize);
        let mut houses = houses;
        houses.sort();
        let mut med_sum = vec![vec![0;n]; n];
        for i in (0..(n-1)).rev() {
            for j in (i + 1)..n {
                med_sum[i ][j] = med_sum[i + 1][j - 1] + houses[j] - houses[i];
            }
        }
        let mut f = vec![vec![i32::MAX / 2; k + 1]; n];
        for i in 0..n {
            f[i][1]=med_sum[0][i];
            for j in 2..=k.min(i + 1) {
                for i0 in 0..i {
                    f[i][j] = f[i][j].min(f[i0][j - 1] + med_sum[i0 + 1][i])
                }
            }
        }
        f[n - 1][k]
    }
}
// @lc code=end
