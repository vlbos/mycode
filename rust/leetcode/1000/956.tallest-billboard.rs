/*
 * @lc app=leetcode id=956 lang=rust
 *
 * [956] Tallest Billboard
 */

// @lc code=start
impl Solution {
    pub fn tallest_billboard(rods: Vec<i32>) -> i32 {
        let n = rods.len();
        let mut memo = vec![vec![i32::MAX / 3; 10001]; n];
        fn dp(rods: &Vec<i32>, i: usize, s: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
            if i == rods.len() {
                return if s == 5000 { 0 } else { i32::MIN / 3 };
            }
            if memo[i][s] != i32::MAX / 3 {
                return memo[i][s];
            }
            let mut ans = dp(rods, i + 1, s, memo);
            ans = ans.max(dp(rods, i + 1, s - rods[i] as usize, memo));
            ans = ans.max(rods[i] + dp(rods, i + 1, s + rods[i] as usize, memo));
            memo[i][s] = ans;
            ans
        }
        dp(&rods, 0, 5000, &mut memo)
    }
}
// @lc code=end
