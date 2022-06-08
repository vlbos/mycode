/*
 * @lc app=leetcode id=1655 lang=rust
 *
 * [1655] Distribute Repeating Integers
 */

// @lc code=start
impl Solution {
    pub fn can_distribute(nums: Vec<i32>, quantity: Vec<i32>) -> bool {
        let mut cache = std::collections::HashMap::new();
        for &x in &nums {
            *cache.entry(x).or_insert(0) += 1;
        }
        let cnt: Vec<i32> = cache.values().cloned().collect();
        let (n, m) = (cnt.len(), quantity.len());
        let m1 = 1 << m;
        let mut sum = vec![0; m1];
        for i in 1..m1 {
            for j in 0..m {
                if i & (1 << j) > 0 {
                    let left = i - (1 << j);
                    sum[i] = sum[left] + quantity[j];
                    break;
                }
            }
        }
        let mut dp = vec![vec![false; m1]; n];
        for i in 0..n {
            dp[i][0] = true;
        }
        for i in 0..n {
            for j in 0..m1 {
                if i > 0 && dp[i - 1][j] {
                    dp[i][j] = true;
                    continue;
                }
                let mut k = j;
                while k > 0 {
                    let prev = j - k;
                    let last = if i == 0 { prev == 0 } else { dp[i - 1][prev] };
                    let need = sum[k] <= cnt[i];
                    if last && need {
                        dp[i][j] = true;
                        break;
                    }
                    k = (k - 1) & j;
                }
            }
        }
        dp[n - 1][m1 - 1]
    }
}
// @lc code=end
