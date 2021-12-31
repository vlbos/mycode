/*
 * @lc app=leetcode id=647 lang=rust
 *
 * [647] Palindromic Substrings
 */

// @lc code=start
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let mut sv = "$#".to_string();
        for c in s.chars() {
            sv += &c.to_string();
            sv += "#";
        }
        sv += "!";
        let n = sv.len();
        let sv = sv.chars().collect::<Vec<char>>();
        let mut ans = 0;
        let mut imax = 0;
        let mut rmax = 0;
        let mut f = vec![0; n];
        for i in 1..n {
            f[i] = if i <= rmax {
                (rmax - i + 1).min(f[(2 * imax - i) as usize])
            } else {
                1
            };
            while i + f[i]<n && i>=f[i] && sv[i + f[i]] == sv[i - f[i]] {
                f[i] += 1;
            }
            if i + f[i] - 1 > rmax {
                imax = i;
                rmax =  i + f[i] - 1;
            }
            ans += f[i] / 2;
        }
        ans as _
    }
}
// @lc code=end
