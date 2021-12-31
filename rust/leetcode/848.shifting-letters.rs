/*
 * @lc app=leetcode id=848 lang=rust
 *
 * [848] Shifting Letters
 */

// @lc code=start
impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<i32>) -> String {
        let n = shifts.len();
        let mut pre = vec![0; n];
        for i in (0..n).rev() {
            pre[i] = (pre[i]+shifts[i] + if i <n-1 { pre[i+1] } else { 0 }) % 26;
        }
        let mut ans = String::new();
        for (i, b) in s.bytes().enumerate() {
            ans.push((b'a'+(b-b'a' + pre[i] as u8)%26) as char);
        }
        ans
    }
}
// @lc code=end
