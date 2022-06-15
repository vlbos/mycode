/*
 * @lc app=leetcode id=2138 lang=rust
 *
 * [2138] Divide a String Into Groups of Size k
 */

// @lc code=start
impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        let mut ans = Vec::new();
        let k = k as usize;
        let n = s.len();
        for i in (0..n).step_by(k as usize) {
            ans.push(s[i..n.min(i + k)].to_string());
        }
        if n % k > 0 {
            ans[n / k].push_str(fill.to_string().repeat(k - n % k).as_str());
        }
        ans
    }
}
// @lc code=end
