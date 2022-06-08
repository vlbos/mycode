/*
 * @lc app=leetcode id=1449 lang=rust
 *
 * [1449] Form Largest Integer With Digits That Add up to Target
 */

// @lc code=start
impl Solution {
    pub fn largest_number(cost: Vec<i32>, target: i32) -> String {
       let target = target as usize;
        let mut dp = vec![i32::MIN; target + 1];
        dp[0] = 0;
        for &c in &cost {
            let c = c as usize;
            for j in c..=target {
                dp[j] = dp[j].max(dp[j - c] + 1);
            }
        }
        if dp[target] < 0 {
            return "0".to_string();
        }
        let mut ans = String::new();
        let mut j = target;
        for i in (0..9).rev() {
            let c = cost[i] as usize;
            while j >= c && dp[j] == dp[j - c] + 1 {
                ans.push((b'1' + i as u8) as char);
                j -= c;
            }
        }
        ans
    }
}
// @lc code=end
