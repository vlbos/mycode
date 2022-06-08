/*
 * @lc app=leetcode id=2111 lang=rust
 *
 * [2111] Minimum Operations to Make the Array K-Increasing
 */

// @lc code=start
impl Solution {
    pub fn k_increasing(arr: Vec<i32>, k: i32) -> i32 {
        let (n, k) = (arr.len(), k as usize);
        let mut ans = 0;
        for i in 0..k {
            let mut f = Vec::new();
            let mut length = 0;
            for j in (i..n).step_by(k) {
                length += 1;
                let g = f.partition_point(|&x| x <= arr[j]);
                if g == f.len() {
                    f.push(arr[j]);
                } else {
                    f[g] = arr[j];
                }
            }
            ans += length - f.len() as i32;
        }
        ans
    }
}
// @lc code=end
