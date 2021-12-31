/*
 * @lc app=leetcode id=1351 lang=rust
 *
 * [1351] Count Negative Numbers in a Sorted Matrix
 */

// @lc code=start
impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for g in &grid {
            for n in g.iter().rev() {
                if *n < 0 {
                    ans += 1;
                } else {
                    break;
                }
            }
        }
        ans
    }
}
// @lc code=end
