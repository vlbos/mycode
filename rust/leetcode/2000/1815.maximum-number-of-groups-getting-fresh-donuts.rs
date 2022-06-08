/*
 * @lc app=leetcode id=1815 lang=rust
 *
 * [1815] Maximum Number of Groups Getting Fresh Donuts
 */

// @lc code=start
impl Solution {
    pub fn max_happy_groups(batch_size: i32, groups: Vec<i32>) -> i32 {
        let mut freq0 = vec![0; 9];
        let mut freq = vec![0; 9];
        for &g in &groups {
            freq0[(g % batch_size) as usize] += 1;
        }
        let mut msum = freq0[1..].iter().map(|x| *x + 1).product::<i32>();
        let b = batch_size as usize;
        let mut w = vec![0; 9];
        w[1] = 1;
        for i in 2..b {
            w[i] = w[i - 1] * (freq0[i - 1] + 1);
        }
        let n = msum as usize;
        let mut f = vec![0; n];
        for fmask in 1..n {
            let mut last = 0;
            for i in 1..b {
                freq[i] = (fmask as i32/ w[i]) % (freq0[i] + 1);
                last = (last + (freq0[i] - freq[i]) * i  as i32) % b  as i32;
            }
            for i in 1..b {
                if freq[i] > 0 {
                    f[fmask] = f[fmask].max(f[fmask - w[i] as usize] + if last == 0 { 1 } else { 0 });
                }
            }
        }
        f[n - 1] + freq0[0]
    }
}
// @lc code=end
