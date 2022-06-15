/*
 * @lc app=leetcode id=2133 lang=rust
 *
 * [2133] Check if Every Row and Column Contains All Numbers
 */

// @lc code=start
impl Solution {
    pub fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
        let n = matrix.len();
        let mut occ = std::collections::HashSet::new();
        for (i, row) in matrix.iter().enumerate() {
            occ.clear();
            for (j, &v) in row.iter().enumerate() {
                if occ.contains(&v) {
                    return false;
                }
                occ.insert(v);
            }
        }
        for j in 0..n {
            occ.clear();
            for i in 0..n {
                let v = matrix[i][j];
                if occ.contains(&v) {
                    return false;
                }
                occ.insert(v);
            }
        }
        true
    }
}
// @lc code=end
