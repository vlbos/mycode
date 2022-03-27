/*
 * @lc app=leetcode id=2059 lang=rust
 *
 * [2059] Minimum Operations to Convert Number
 */

// @lc code=start
impl Solution {
    pub fn minimum_operations(nums: Vec<i32>, start: i32, goal: i32) -> i32 {
        let mut q = std::collections::VecDeque::new();
        let mut vis = vec![false; 1001];
        q.push_back((start, 0));
        vis[start as usize] = true;
        while let Some((x, step)) = q.pop_front() {
            for &v in &nums {
                let nxes = vec![x + v, x - v, x ^ v];
                for &nx in &nxes {
                    if nx == goal {
                        return step + 1;
                    }
                    if nx >= 0 && nx <= 1000 && !vis[nx as usize] {
                        q.push_back((nx, step+1));
                        vis[nx as usize] = true;
                    }
                }
            }
        }
        -1
    }
}
// @lc code=end
