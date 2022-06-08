/*
 * @lc app=leetcode id=1711 lang=rust
 *
 * [1711] Count Good Meals
 */

// @lc code=start
impl Solution {
    pub fn count_pairs(deliciousness: Vec<i32>) -> i32 {
        let maxsum = deliciousness.iter().max().unwrap() * 2;
        let mut m = std::collections::HashMap::new();
        let mut ans = 0;
        for &v in &deliciousness {
            let mut sum = 1;
            while sum <= maxsum {
                ans += *m.get(&(sum - v)).unwrap_or(&0);
                ans %= 1000_000_007;
                sum <<= 1;
            }
            *m.entry(v).or_insert(0) += 1;
        }
        ans
    }
}
// @lc code=end
