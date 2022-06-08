/*
 * @lc app=leetcode id=1899 lang=rust
 *
 * [1899] Merge Triplets to Form Target Triplet
 */

// @lc code=start
impl Solution {
    pub fn merge_triplets(triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let mut ans = vec![0; target.len()];
        for t in triplets {
            if t.iter().zip(&target).all(|(x, y)| *x <= *y) {
                ans = vec![ans[0].max(t[0]), ans[1].max(t[1]), ans[2].max(t[2])];
            }
        }
        ans == target
    }
}
// @lc code=end
