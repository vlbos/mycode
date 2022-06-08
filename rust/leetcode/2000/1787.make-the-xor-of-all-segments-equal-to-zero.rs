/*
 * @lc app=leetcode id=1787 lang=rust
 *
 * [1787] Make the XOR of All Segments Equal to Zero
 */

// @lc code=start
impl Solution {
    pub fn min_changes(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let m = 1 << 10;
        let mut f = vec![i32::MAX / 2; m];
        f[0] = 0;
        let k = k as usize;
        for i in 0..k {
            let mut cnt = std::collections::HashMap::new();
            let mut size = 0;
            for j in (i..n).step_by(k) {
                *cnt.entry(nums[j]).or_insert(0) += 1;
                size += 1;
            }
            let t2min = *f.iter().min().unwrap();
            let mut g = vec![t2min; m];
            for mask in 0..m {
                for (&x, &countx) in &cnt {
                    g[mask] = g[mask].min(f[mask ^ x as usize] - countx);
                }
            }
            f = g.into_iter().map(|v| v + size).collect();
        }
        f[0]
    }
}
// @lc code=end
