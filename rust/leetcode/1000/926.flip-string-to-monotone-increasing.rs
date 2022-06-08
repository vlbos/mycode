/*
 * @lc app=leetcode id=926 lang=rust
 *
 * [926] Flip String to Monotone Increasing
 */

// @lc code=start
impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        let n = s.len();
        let mut pre = vec![0;n+1];
        for i in 0..n{
            pre[i+1]=pre[i]+(s.bytes().nth(i).unwrap()-b'0') as i32;
        }
        let mut ans =i32::MAX;
        for i in 0..=n{
            ans = ans.min(pre[i]+(n-i) as i32-(pre[n]-pre[i]));
        }
        ans
    }
}
// @lc code=end

