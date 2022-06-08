/*
 * @lc app=leetcode id=1643 lang=rust
 *
 * [1643] Kth Smallest Instructions
 */

// @lc code=start
impl Solution {
    pub fn kth_smallest_path(destination: Vec<i32>, k: i32) -> String {
        let (mut h, mut v) = (destination[1] as usize, destination[0] as usize);
        let mut comb = vec![vec![0; h]; h + v];
        comb[0][0] = 1;
        for i in 1..h + v {
            comb[i][0] = 1;
            for j in 1..=i.min(h - 1) {
                comb[i][j] = comb[i - 1][j - 1] + comb[i - 1][j];
            }
        }
        let mut k = k;
        let mut ans = String::new();
        let n = h + v;
        for i in 0..n {
            if h == 0 {
                ans.push('V');
                v -= 1;
                continue;
            }
            let o = comb[h + v - 1][h - 1];
            if k > o {
                ans.push('V');
                v -= 1;
                k -= o;
            } else {
                ans.push('H');
                h -= 1;
            }
        }
        ans
    }
}
// @lc code=end
