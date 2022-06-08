/*
 * @lc app=leetcode id=1105 lang=rust
 *
 * [1105] Filling Bookcase Shelves
 */

// @lc code=start
impl Solution {
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        let n = books.len();
        let mut dp = vec![1000_000; n + 1];
        dp[0] = 0;
        for i in 1..n + 1 {
            let (mut w, mut h) = (0, 0);
            for j in (1..=i).rev() {
                w += books[j - 1][0];
                if w > shelf_width {
                    break;
                }
                h = h.max(books[j - 1][1]);
                dp[i] = dp[i].min(dp[j - 1] + h);
            }
        }
        dp[n]
    }
}
// @lc code=end
