/*
 * @lc app=leetcode id=1636 lang=rust
 *
 * [1636] Sort Array by Increasing Frequency
 */

// @lc code=start
impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut b = vec![Vec::new(); nums.len() + 1];
        let mut f = vec![0; 201];
        for n in &nums {
            f[*n as usize + 100] += 1;
        }
        for (i, n) in f.iter().enumerate() {
            b[*n as usize].insert(0,i as i32 - 100);
        }
        let mut ans = Vec::new();
        for (i, v) in b.iter().enumerate() {
            if i > 0 && !v.is_empty() {
                for n in v {
                    ans.extend(vec![n; i]);
                }
            }
        }
        ans
    }
}
// @lc code=end
