/*
 * @lc app=leetcode id=940 lang=rust
 *
 * [940] Distinct Subsequences II
 */

// @lc code=start
impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        let mut dp = vec![1i64];
        let mut last = std::collections::HashMap::new();
        for (i, x) in s.chars().enumerate() {
            let l = dp[dp.len() - 1];
            dp.push((l * 2) % 1_000_000_007);
            if last.contains_key(&x) {
                let l = dp[*last.get(&x).unwrap()];
                *dp.last_mut().unwrap() -= l;
            }
            *dp.last_mut().unwrap() %= 1_000_000_007;
            last.insert(x, i);
        }
        *dp.last_mut().unwrap() -= 1;
        (if *dp.last().unwrap() < 0 {
            *dp.last().unwrap() + 1_000_000_007
        } else {
            dp[dp.len() - 1] % 1_000_000_007
        }) as _
    }
}
// @lc code=end
