/*
 * @lc app=leetcode id=218 lang=rust
 *
 * [218] The Skyline Problem
 */

// @lc code=start
impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut boundaries: Vec<i32> = buildings
            .iter()
            .map(|x| vec![x[0], x[1]])
            .flatten()
            .collect();
        boundaries.sort();
        let mut ans:Vec<Vec<i32>>  = Vec::new();
        let n = buildings.len();
        let mut idx = 0;
        let mut q = std::collections::BinaryHeap::new();
        for &b in &boundaries {
            while idx < n && buildings[idx][0] <= b {
                q.push(vec![buildings[idx][2], buildings[idx][1]]);
                idx += 1;
            }
            while !q.is_empty() && q.peek().unwrap()[1] <= b {
                q.pop();
            }
            let maxn = if q.is_empty() {
                0
            } else {
                q.peek().unwrap()[0]
            };
            if ans.is_empty() || maxn != ans[ans.len() - 1][1] {
                ans.push(vec![b, maxn]);
            }
        }
        ans
    }
}
// @lc code=end
