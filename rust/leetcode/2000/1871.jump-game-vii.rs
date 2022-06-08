/*
 * @lc app=leetcode id=1871 lang=rust
 *
 * [1871] Jump Game VII
 */

// @lc code=start
impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let n = s.len();
        let bs = s.as_bytes();
        let mut f = vec![0; n];
        f[0] = 1;
        let (m, x) = (min_jump as usize, max_jump as usize);
        let mut pre = vec![1; m];
        pre.extend(vec![0; n - m]);
        for i in m..n {
            let (l, r) = (if i>x {i - x}else{0}, i - m);
            if bs[i] == b'0' {
                let t = pre[r] - if l >0 { pre[l - 1]  } else {0 };
                f[i] = if t > 0 { 1 } else { 0 };
            }
            pre[i] = pre[i - 1] + f[i];
        }
        f[n - 1] > 0
    }
}
// @lc code=end
