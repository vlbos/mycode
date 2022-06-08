/*
 * @lc app=leetcode id=1284 lang=rust
 *
 * [1284] Minimum Number of Flips to Convert Binary Matrix to Zero Matrix
 */

// @lc code=start
impl Solution {
    pub fn min_flips(mat: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (mat.len(), mat[0].len());

        let convert = |mat: &mut Vec<Vec<i32>>, i: usize, j: usize| {
            let dirs = [0, 0, 1, 0, -1, 0];
            for k in 0..dirs.len() - 1 {
                let (ii, jj) = (i as i32 + dirs[k], j as i32 + dirs[k + 1]);
                if ii >= 0 && ii < m as i32 && jj >= 0 && jj < n as i32 {
                    mat[ii as usize][jj as usize] ^= 1;
                }
            }
        };

        let mut ans = i32::MAX;
        for bin in 0..(1 << n) {
            let mut mat_copy = mat.clone();
            let mut flip_cnt = 0;
            for j in 0..n {
                if bin & (1 << j) > 0 {
                    flip_cnt += 1;
                    convert(&mut mat_copy, 0, j);
                }
            }
            for i in 1..m {
                for j in 0..n {
                    if mat_copy[i - 1][j] == 1 {
                        flip_cnt += 1;
                        convert(&mut mat_copy, i, j);
                    }
                }
            }
            if mat_copy[m - 1].iter().all(|x| *x == 0) {
                ans = ans.min(flip_cnt);
            }
        }
        if ans != i32::MAX {
            ans
        } else {
            -1
        }
    }
}
// @lc code=end
