/*
 * @lc app=leetcode id=1962 lang=rust
 *
 * [1962] Remove Stones to Minimize the Total
 */

// @lc code=start
impl Solution {
    pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        let mut q = piles
            .iter()
            .cloned()
            .collect::<std::collections::BinaryHeap<i32>>();
        for _ in 0..k {
            if let Some(mut n) = q.peek_mut() {
                *n-=(*n)/2;
            }
        }
        q.iter().sum::<i32>()
    }
}
// @lc code=end
