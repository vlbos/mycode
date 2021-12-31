/*
 * @lc app=leetcode id=15 lang=rust
 *
 * [15] 3Sum
 */

// @lc code=start
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut n = nums;
        n.sort();
        let mut ans = Vec::new();

        for i in 0..n.len() {
            if i > 0 && n[i] == n[i - 1] {
                continue;
            }
            let mut k = n.len() - 1;
            let target = -n[i];
            for j in i + 1..n.len() {
                if j > i + 1 && n[j] == n[j - 1] {
                    continue;
                }

                while j < k && n[j] + n[k] > target {
                    k -= 1;
                }
                if j == k {
                    break;
                }
                if n[j] + n[k] == target {
                    ans.push(vec![n[i], n[j], n[k]]);
                }
            }
        }

        ans
    }
}
// @lc code=end
