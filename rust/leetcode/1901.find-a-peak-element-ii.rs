/*
 * @lc app=leetcode id=1901 lang=rust
 *
 * [1901] Find a Peak Element II
 */

// @lc code=start
impl Solution {
    pub fn find_peak_grid(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (mat.len(), mat[0].len());
        let (mut up, mut down) = (0, m - 1);
        while up <= down {
            let mid = (up + down) / 2;
            let maxup = if mid == 0 {
                -1
            } else {
                *mat[mid - 1].iter().max().unwrap()
            };
            let maxdown = if mid == m - 1 {
                -1
            } else {
                *mat[mid + 1].iter().max().unwrap()
            };
            let max = *mat[mid].iter().max().unwrap();
            if max >= maxup.max(maxdown) {
                return vec![
                    mid as i32,
                    mat[mid].iter().position(|x| *x == max).unwrap() as i32,
                ];
            }
            if maxup >= max.max(maxdown) {
                down = mid;
            } else {
                up = mid + 1;
            }
        }
        Vec::new()
    }
}
// @lc code=end
