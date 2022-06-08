/*
 * @lc app=leetcode id=52 lang=rust
 *
 * [52] N-Queens II
 */

// @lc code=start
impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        fn solve(n: i32, row: i32, columns: i32, diagonals1: i32, diagonals2: i32) -> i32 {
            if row == n {
                return 1;
            }
            let mut ans = 0;

            let mut ap = ((1 << n) - 1) & (!(columns | diagonals1 | diagonals2));
            while ap != 0 {
                let p = ap & (-ap);
                ap &= (ap - 1);
                ans += solve(
                    n,
                    row + 1,
                    columns | p,
                    (diagonals1 | p) >> 1,
                    (diagonals2 | p) << 1,
                );
            }
            ans
        }
        solve(n, 0, 0, 0, 0)
    }
}
// @lc code=end
