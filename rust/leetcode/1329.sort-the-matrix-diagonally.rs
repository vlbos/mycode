/*
 * @lc app=leetcode id=1329 lang=rust
 *
 * [1329] Sort the Matrix Diagonally
 */

// @lc code=start
impl Solution {
    pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::BinaryHeap;
        use std::collections::HashMap;
        let (m, n) = (mat.len(), mat[0].len());
        let mut mat = mat;
        let mut d: HashMap<i32, BinaryHeap<i32>> = HashMap::new();
        (0..m).for_each(|i| {
            (0..n).for_each(|j| d.entry((i - j) as i32).or_default().push(-mat[i][j]))
        });
        (0..m).for_each(|i| {
            (0..n).for_each(|j| mat[i][j] = -d.get_mut(&((i - j) as i32)).unwrap().pop().unwrap())
        });
        mat
    }
}
// @lc code=end
