/*
 * @lc app=leetcode id=1337 lang=rust
 *
 * [1337] The K Weakest Rows in a Matrix
 */

// @lc code=start
impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut ans = Vec::new();
        for j in 0..mat[0].len() {
            for i in 0..mat.len() {
                let ii = i as i32;
                if mat[i][j] == 0 && !ans.contains(&ii) {
                    ans.push(ii);
                    if ans.len() == k as usize {
                        return ans;
                    }
                }
            }
        }

        if ans.is_empty() {
            ans.push(0);
        } else if ans.len() < k as usize {
            let j = mat[0].len() - 1;
            for i in 0..mat.len() {
                let ii = i as i32;
                if mat[i][j] == 1 && !ans.contains(&ii) {
                    ans.push(ii);
                    if ans.len() == k as usize {
                        return ans;
                    }
                }
            }
        }
        ans
    }
}
// @lc code=end
