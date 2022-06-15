/*
 * @lc app=leetcode id=1995 lang=rust
 *
 * [1995] Count Special Quadruplets
 */

// @lc code=start
impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = 0;
        let mut cnt = std::collections::HashMap::new();
        for b in (1..n - 2).rev() {
            for d in b + 2..n {
                *cnt.entry(nums[d] - nums[b + 1]).or_insert(0) += 1;
            }
            for a in 0..b {
                let sum = nums[a] + nums[b];
                if let Some(&v) = cnt.get(&sum) {
                    ans += v;
                }
            }
        }
        ans
    }
}
// @lc code=end
