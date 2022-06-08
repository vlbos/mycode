/*
 * @lc app=leetcode id=2023 lang=rust
 *
 * [2023] Number of Pairs of Strings With Concatenation Equal to Target
 */

// @lc code=start
impl Solution {
    pub fn num_of_pairs(nums: Vec<String>, target: String) -> i32 {
        let mut cnt = std::collections::HashMap::new();
        for v in &nums {
            *cnt.entry(v).or_insert(0) += 1;
        }
        let mut ans = 0;
        for i in 1..target.len() {
            let mut p = target.clone();
            let s = p.split_off(i);
            if p != s {
                ans += (*cnt.get(&p).unwrap_or(&0)) * (*cnt.get(&s).unwrap_or(&0));
            } else {
                let n = *cnt.get(&p).unwrap_or(&0);
                ans += n * (n - 1);
            }
        }
        ans
    }
}
// @lc code=end
