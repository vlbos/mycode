/*
 * @lc app=leetcode id=403 lang=rust
 *
 * [403] Frog Jump
 */

// @lc code=start
impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        let n = stones.len();
        let mut dp = vec![vec![false; n]; n];
        dp[0][0] = true;
        for i in 1..n {
            if stones[i] - stones[i - 1] > i as i32{
                return false;
            }
        }
        for i in 1..n {
            for j in (0..i).rev() {
                let k = (stones[i] - stones[j]) as usize;
                if k > j + 1 {
                    break;
                }
                dp[i][k] = dp[j][k - 1] | dp[j][k] | if k+1<n {dp[j][k + 1]} else{false};
                if i == n - 1 && dp[i][k] {
                    return true;
                }
            }
        }
        false
    }
}
// @lc code=end
