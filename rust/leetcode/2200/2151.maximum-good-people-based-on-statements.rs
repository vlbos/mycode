/*
 * @lc app=leetcode id=2151 lang=rust
 *
 * [2151] Maximum Good People Based on Statements
 */

// @lc code=start
impl Solution {
    pub fn maximum_good(statements: Vec<Vec<i32>>) -> i32 {
        let n = statements.len();
        let mut ans = 0;
        let check = |mask: i32| {
            for i in 0..n {
                for j in 0..n {
                    if i == j {
                        continue;
                    }
                    if statements[i][j] == 0 && mask & (1 << i) > 0 && mask & (1 << j) > 0
                        || statements[i][j] == 1 && mask & (1 << i) > 0 && mask & (1 << j) == 0
                    {
                        return false;
                    }
                }
            }
            true
        };
        for mask in 1..(1 << n) {
            if check(mask) {
                ans = ans.max(mask.count_ones())
            }
        }
        ans as _
    }
}
// @lc code=end
