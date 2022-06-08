/*
 * @lc app=leetcode id=1864 lang=rust
 *
 * [1864] Minimum Number of Swaps to Make the Binary String Alternating
 */

// @lc code=start
impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let bs = s.as_bytes();
        let mut ans = i32::MAX;
        let (mut i, mut j) = (0, 0);
        let n = bs.len();
        let mut nn = vec![0; 2];
        for b in bs {
            nn[(b - b'0') as usize] += 1;
        }
        if nn[1] == (n + 1) / 2 && nn[0] == n / 2 {
            let mut d1 = 0;
            for i in 0..n {
                if (bs[i] - b'0') as usize == i % 2 {
                    d1 += 1;
                }
            }
            ans = ans.min(d1 / 2);
        }
        if nn[0] == (n + 1) / 2 && nn[1] == n / 2 {
            let mut d1 = 0;
            for i in 0..n {
                if (bs[i] - b'0') as usize != i % 2 {
                    d1 += 1;
                }
            }
            ans = ans.min(d1 / 2);
        }
        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
}
// @lc code=end
