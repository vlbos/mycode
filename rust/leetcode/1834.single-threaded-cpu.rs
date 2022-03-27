/*
 * @lc app=leetcode id=1834 lang=rust
 *
 * [1834] Single-Threaded CPU
 */

// @lc code=start
impl Solution {
    pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        let n = tasks.len();
        let mut indices: Vec<usize> = (0..n).collect();
        indices.sort_by_key(|x| tasks[*x][0]);
        let mut tm = 0;
        let mut q = std::collections::BinaryHeap::new();
        let mut ans = Vec::new();
        let mut i = 0;
        for _ in 0..n {
            if q.is_empty() {
                tm = tm.max(tasks[indices[i]][0]);
            }
            while i < n && tasks[indices[i]][0] <= tm {
                q.push((-tasks[indices[i]][1], indices[i] as i32*-1));
                i += 1;
            }
            if let Some((process, index)) = q.pop() {
                tm -= process;
                ans.push(index*-1);
            }
        }
        ans
    }
}
// @lc code=end
