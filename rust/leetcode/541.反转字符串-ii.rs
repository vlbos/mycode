/*
 * @lc app=leetcode.cn id=541 lang=rust
 *
 * [541] 反转字符串 II
 */

// @lc code=start
impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let mut sb = s.into_bytes();
        let mut i = 0usize;
        let k = k as usize;
        if i + k <= sb.len() {
            while i <= sb.len() {
                if i + k <= sb.len() {
                    for j in 0..k / 2 {
                        let jj = i + j;
                        let t = sb[jj];
                        sb[jj] = sb[i + k - 1 - j];
                        sb[i + k - 1 - j] = t;
                    }
                } else {
                    let k = sb.len() - i;
                    for j in 0..k / 2 {
                        let jj = i + j;
                        let t = sb[jj];
                        sb[jj] = sb[i + k - 1 - j];
                        sb[i + k - 1 - j] = t;
                    }
                }
                i += 2 * k;
            }
        } else {
            let k = sb.len() - i;
            for j in 0..k / 2 {
                let jj = i + j;
                let t = sb[jj];
                sb[jj] = sb[i + k - 1 - j];
                sb[i + k - 1 - j] = t;
            }
        }

        String::from_utf8(sb).unwrap()
    }
}
// @lc code=end
