/*
 * @lc app=leetcode id=2028 lang=rust
 *
 * [2028] Find Missing Observations
 */

// @lc code=start
impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let m = rolls.len() as i32;
        let sum = rolls.iter().sum::<i32>();
        let nm = mean * (m + n);
        if nm < sum || nm - sum < n || nm - sum > n * 6 {
            return Vec::new();
        }
        let mut s = nm - sum;
        let mut ans = vec![s / n; n as usize];
        s %= n;
        for i in 0..s as usize {
            ans[i] += 1;
        }
        ans
    }
}
// @lc code=end
