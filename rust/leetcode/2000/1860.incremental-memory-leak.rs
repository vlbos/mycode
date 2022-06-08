/*
 * @lc app=leetcode id=1860 lang=rust
 *
 * [1860] Incremental Memory Leak
 */

// @lc code=start
impl Solution {
    pub fn mem_leak(memory1: i32, memory2: i32) -> Vec<i32> {
        let mut t = 1;
        let mut memory1 = memory1;
        let mut memory2 = memory2;
        while t <= memory1.max(memory2) {
            if memory1 < memory2 {
                memory2 -= t;
            } else {
                memory1 -= t;
            }
            t+=1;
        }
        vec![t, memory1, memory2]
    }
}
// @lc code=end
