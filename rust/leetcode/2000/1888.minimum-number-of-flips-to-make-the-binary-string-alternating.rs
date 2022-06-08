/*
 * @lc app=leetcode id=1888 lang=rust
 *
 * [1888] Minimum Number of Flips to Make the Binary String Alternating
 */

// @lc code=start
impl Solution {
    pub fn min_flips(s: String) -> i32 {
        let n = s.len();
        let mut pre = vec![vec![0, 0]; n];
        let b2i = |c: u8, x: u8| -> i32 {
            if c - b'0' == x {
                1
            } else {
                0
            }
        };
        let bs = s.as_bytes();
         pre[0]=  vec![b2i(bs[0], 1),b2i(bs[0], 0)];
        for i in 1..n {
            pre[i][0] = pre[i - 1][1] + b2i(bs[i], 1);
            pre[i][1] = pre[i - 1][0] + b2i(bs[i], 0);
        }
        let mut ans = pre[n - 1][0].min(pre[n - 1][1]);
        if n % 2 > 0 {
            let mut suf = vec![vec![0, 0]; n];
            suf[n-1] = vec![b2i(bs[n-1], 1),b2i(bs[n-1], 0)];
            for i in (0..n-1).rev() {
                suf[i][0] = suf[i + 1][1] + b2i(bs[i], 1);
                suf[i][1] = suf[i + 1][0] + b2i(bs[i], 0);
            }
            for i in 0..n - 1 {
                ans = ans.min(pre[i][0] + suf[i + 1][0]);
                ans = ans.min(pre[i][1] + suf[i + 1][1]);
            }
        }
        ans
    }
}
// @lc code=end
