/*
 * @lc app=leetcode id=1886 lang=rust
 *
 * [1886] Determine Whether Matrix Can Be Obtained By Rotation
 */

// @lc code=start
impl Solution {
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        if mat == target {
            return true;
        }
        let t = target.iter().map(|x| x.iter().sum::<i32>()).sum::<i32>();
        let m = mat.iter().map(|x| x.iter().sum::<i32>()).sum::<i32>();
        if t != m {
            return false;
        }
        let trans = |mm: Vec<Vec<i32>>| -> Vec<Vec<i32>> {
            let mut mmm = vec![vec![0; mm.len()]; mm[0].len()];
            for j in 0..mm[0].len() {
                for i in 0..mm.len() {
                    mmm[j ][mm.len()-i-1] = mm[i][j];
                }
            }
            mmm
        };
        let mut mt = mat.clone();
        for k in 0..3 {
            mt = trans(mt);
            if mt == target {
                return true;
            }
        }
        false
    }
}
// @lc code=end
