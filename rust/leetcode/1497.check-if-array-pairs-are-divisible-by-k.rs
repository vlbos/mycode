/*
 * @lc app=leetcode id=1497 lang=rust
 *
 * [1497] Check If Array Pairs Are Divisible by k
 */

// @lc code=start
impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let kk = k as usize;
        let mut m = vec![0; kk];
        for &a in &arr {
            m[((a % k + k) % k) as usize] += 1;
        }
        for i in 1..=kk / 2 {
            if m[i] != m[kk - i] {
                return false;
            }
        }
        m[0] % 2 == 0
    }
}
// @lc code=end
