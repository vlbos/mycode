/*
 * @lc app=leetcode id=1981 lang=rust
 *
 * [1981] Minimize the Difference Between Target and Chosen Elements
 */

// @lc code=start
impl Solution {
    pub fn minimize_the_difference(mat: Vec<Vec<i32>>, target: i32) -> i32 {
        let t = target as usize;
        let mut f = vec![false; t];
        f[0] = true;
        let mut large = i32::MAX;
        for r in &mat {
            let mut next_large = i32::MAX;
            let mut g = vec![false; t];
            for &x in r {
                for j in 0..t {
                    if f[j] {
                        if j as i32 + x >= target {
                            next_large = next_large.min(j as i32 + x);
                        } else {
                            g[j + x as usize] = true;
                        }
                    }
                }
                if large != i32::MAX {
                    next_large = next_large.min(large + x);
                }
            }
            f = g;
            large = next_large;
        }
        let mut ans = (large - target).abs();
        for i in (0..t).rev() {
            if f[i] {
                ans = ans.min((t - i) as i32);
                break;
            }
        }
        ans
    }
}
// @lc code=end
