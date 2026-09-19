// 3967. Finish Time of Tasks II

// ## Description
// You are given an integer `n` representing the number of tasks in a project, numbered from 0 to `n - 1`.
// These tasks are connected as an undirected** tree**.
// This is represented by a 2D integer array `edges` of length `n - 1`,
//  where `edges[i] = [ui, vi]` indicates an undirected connection between task `ui` and task `vi`.
// You are also given an array `baseTime` of length `n`, where `baseTime[i]` represents the time to complete task `i`.
// For any chosen task as the root, the **finish time** of each task is calculated as follows:
// * Leaf task: The finish time is `baseTime[i]`.
// * Non-leaf task:
// * Let `earliest` be the **minimum** finish time among its children,
// and `latest` be the **maximum** finish time among its children.
// * Let `ownDuration` be `(latest - earliest) + baseTime[i]`.
// * Finish time of task `i` is `latest + ownDuration`.
// Choose **any** task as the root and compute the finish time of that root based on the rules above.
// Return the **minimum** possible finish time among all choices of root.

// **Example 1:**
// **Input:** n = 3, edges = [[0,1],[1,2]], baseTime = [9,1,5]
// **Output:** 14
// **Explanation:**
// 0 9 1 1 2 5
// The optimal choice is to treat task 1 as the root.
// * Task 0 is a leaf, so its finish time is `baseTime[0] = 9`.
// * Task 2 is a leaf, so its finish time is `baseTime[2] = 5`.
// * Task 1 has two children with finish times 9 and 5:
// * `earliest = 5`, `latest = 9`
// * `ownDuration = (latest - earliest) + baseTime[1] = (9 - 5) + 1 = 5`
// * Finish time of task 1 is `latest + ownDuration = 9 + 5 = 14`
// Thus, the minimum possible finish time among all choices of root is 14.
// **Example 2:**
// **Input:** n = 3, edges = [[0,1],[0,2]], baseTime = [4,7,6]
// **Output:** 12
// **Explanation:**
// 0 4 1 7 2 6
// The optimal choice is to treat task 0 as the root.
// * Task 1 is a leaf, so its finish time is `baseTime[1] = 7`.
// * Task 2 is a leaf, so its finish time is `baseTime[2] = 6`.
// * Task 0 has two children with finish times 7 and 6:
// * `earliest = 6`, `latest = 7`
// * `ownDuration = (latest - earliest) + baseTime[0] = (7 - 6) + 4 = 5`
// * Finish time of task 0 is `latest + ownDuration = 7 + 5 = 12`
// Thus, the minimum possible finish time among all choices of root is 12.
// **Example 3:**
// **Input:** n = 4, edges = [[0,1],[0,2],[2,3]], baseTime = [5,8,2,1]
// **Output:** 16
// **Explanation:**
// 0 5 1 8 2 2 3 1
// The optimal choice is to treat task 1 as the root.
// * Task 3 is a leaf, so its finish time is `baseTime[3] = 1`.
// * Task 2 has one child task 3:
// * `earliest = latest = 1`
// * `ownDuration = (latest - earliest) + baseTime[2] = 0 + 2 = 2`
// * Finish time of task 2 is `latest + ownDuration = 1 + 2 = 3`
// * Task 0 has one child task 2:
// * `earliest = latest = 3`
// * `ownDuration = (latest - earliest) + baseTime[0] = 0 + 5 = 5`
// * Finish time of task 0 is `latest + ownDuration = 3 + 5 = 8`
// * Task 1 has one child task 0:
// * `earliest = latest = 8`
// * `ownDuration = (latest - earliest) + baseTime[1] = 0 + 8 = 8`
// * Finish time of task 1 is `latest + ownDuration = 8 + 8 = 16`
// Thus, the minimum possible finish time among all choices of root is 16.

// **Constraints:**
// * `1 \<= n \<= 105`
// * `edges.length = n - 1`
// * `edges[i] == [ui, vi]`
// * `0 \<= ui, vi \<= n - 1`
// * `ui != vi`
// * The input is generated such that `edges` represents a valid undirected tree.
// * `baseTime.length == n`
// * `1 \<= baseTime[i] \<= 105` ## Solutions

//   long long finish_time(int n, vector<vector<int>>& edges, vector<int>& baseTime) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn finish_time(n: i32, edges: Vec<Vec<i32>>, base_time: Vec<i32>) -> i64 {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for e in edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            g[u].push(v);
            g[v].push(u);
        }
        let mut dp = vec![0i64; n];
        let mut s = vec![(1, 0, n)];
        while let Some((step, u, p)) = s.pop() {
            if step == 1 {
                s.push((2, u, p));
                for &v in &g[u] {
                    if v != p {
                        s.push((1, v, u));
                    }
                }
            } else if step == 2 {
                let (mut mx, mut mn) = (i64::MIN, i64::MAX);
                for &v in &g[u] {
                    if v != p {
                        mx = mx.max(dp[v]);
                        mn = mn.min(dp[v]);
                    }
                }
                dp[u] = if mx == i64::MIN { 0 } else { mx * 2 - mn } + base_time[u] as i64;
            }
        }
        let mut ans = i64::MAX;
        let mut s = vec![(0, n, i64::MIN)];
        while let Some((u, p, t)) = s.pop() {
            let (mut mx, mut mn) = (vec![i64::MIN; 2], vec![i64::MAX; 2]);
            for &v in &g[u] {
                let x = if v != p { dp[v] } else { t };
                mx.push(x);
                mx.sort_unstable_by(|a, b| b.cmp(a));
                mx.pop();
                mn.push(x);
                mn.sort_unstable();
                mn.pop();
            }
            ans = ans.min(
                if mx[0] == i64::MIN {
                    0
                } else {
                    mx[0] * 2 - mn[0]
                } + base_time[u] as i64,
            );
            for &v in g[u].iter().rev() {
                if v == p {
                    continue;
                }
                let (mx, mn) = (
                    if dp[v] == mx[0] { mx[1] } else { mx[0] },
                    if dp[v] == mn[0] { mn[1] } else { mn[0] },
                );
                s.push((
                    v,
                    u,
                    if mx == i64::MIN { 0 } else { mx * 2 - mn } + base_time[u] as i64,
                ));
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_matrix;
    #[test]
    pub fn test_finish_time_1() {
        assert_eq!(
            14,
            Solution::finish_time(3, lc_matrix![[0, 1], [1, 2]], vec![9, 1, 5])
        );
    }
    #[test]
    pub fn test_finish_time_2() {
        assert_eq!(
            12,
            Solution::finish_time(3, lc_matrix![[0, 1], [0, 2]], vec![4, 7, 6])
        );
    }
    #[test]
    pub fn test_finish_time_3() {
        assert_eq!(
            16,
            Solution::finish_time(4, lc_matrix![[0, 1], [0, 2], [2, 3]], vec![5, 8, 2, 1])
        );
    }
}
