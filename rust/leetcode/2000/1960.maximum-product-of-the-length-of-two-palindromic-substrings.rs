/*
 * @lc app=leetcode id=1960 lang=rust
 *
 * [1960] Maximum Product of the Length of Two Palindromic Substrings
 */

// @lc code=start
impl Solution {
    pub fn max_product(s: String) -> i64 {
        let n = s.len();
        let mut span = vec![0; n];
        let (mut l, mut r) = (0, -1);
        let bs = s.as_bytes();
        for i in 0..n {
            span[i] = if i as i32 <= r {
                span[(l + r) as usize - i].min((r + 1) as usize - i)
            } else {
                1
            };
            while i >= span[i] && i + span[i] < n && bs[i - span[i]] == bs[i + span[i]] {
                span[i] += 1;
            }
            if (i + span[i]) as i32 > r + 1 {
                l = (i - span[i] + 1) as i32;
                r = (i + span[i] - 1) as i32;
            }
        }
        let mut pre = vec![0; n];
        let mut suf = vec![0; n];
        for i in 0..n {
            pre[i + span[i] - 1] = pre[i + span[i] - 1].max(span[i] as i32 * 2 - 1);
            suf[i - span[i] + 1] = suf[i - span[i] + 1].max(span[i] as i32 * 2 - 1);
        }
        for i in 1..n {
            pre[i] = pre[i].max(pre[i - 1]);
        }
        for i in (0..n - 1).rev() {
            pre[i] = pre[i].max(pre[i + 1] - 2);
        }
        for i in (0..n - 1).rev() {
            suf[i] = suf[i].max(suf[i + 1]);
        }
        for i in 1..n {
            suf[i] = suf[i].max(suf[i - 1] - 2);
        }
        let mut ans = 0;
        for i in 0..n - 1 {
            ans = ans.max((pre[i] as i64) * (suf[i + 1] as i64));
        }
        ans
    }
}
// @lc code=end
