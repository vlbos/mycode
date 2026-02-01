// # 3778. Minimum Distance Excluding One Maximum Weighted Edge 🔒

// Description
// -----------

// You are given a positive integer `n` and a 2D integer array `edges`, where `edges[i] = [ui, vi, wi]`.

// There is a **weighted** **connected** simple undirected graph with `n` nodes labeled from 0 to `n - 1`.
// Each `[ui, vi, wi]` in `edges` represents an edge between node `ui` and node `vi` with **positive** weight `wi`.

// The **cost** of a path is the **sum** of weights of the edges in the path,
// **excluding** the edge with the **maximum** weight.
// If there are multiple edges in the path with the maximum weight, **only** the **first** such edge is excluded.

// Return an integer representing the **minimum** **cost** of a path going from node 0 to node `n - 1`.

// **Example 1:**

// **Input:** n = 5, edges = \[\[0,1,2\],\[1,2,7\],\[2,3,7\],\[3,4,4\]\]

// **Output:** 13

// **Explanation:**

// There is only one path going from node 0 to node 4: `0 -> 1 -> 2 -> 3 -> 4`.

// The edge weights on this path are 2, 7, 7, and 4.

// Excluding the first edge with maximum weight, which is `1 -> 2`, the cost of this path is `2 + 7 + 4 = 13`.

// **Example 2:**

// **Input:** n = 3, edges = \[\[0,1,1\],\[1,2,1\],\[0,2,50000\]\]

// **Output:** 0

// **Explanation:**

// There are two paths going from node 0 to node 2:

// *   `0 -> 1 -> 2`

// The edge weights on this path are 1 and 1.

// Excluding the first edge with maximum weight, which is `0 -> 1`, the cost of this path is 1.

// *   `0 -> 2`

// The only edge weight on this path is 1.

// Excluding the first edge with maximum weight, which is `0 -> 2`, the cost of this path is 0.

// The minimum cost is `min(1, 0) = 0`.

// **Constraints:**

// *   `2 <= n <= 5 * 104`
// *   `n - 1 <= edges.length <= 109`
// *   `edges[i] = [ui, vi, wi]`
// *   `0 <= ui < vi < n`
// *   `[ui, vi] != [uj, vj]`
// *   `1 <= wi <= 5 * 104`
// *   The graph is connected.

// // long long min_cost_excluding_max(int n, vector<vector<int>>& edges) {

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn min_cost_excluding_max(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for e in edges {
            let (u, v, w) = (e[0] as usize, e[1] as usize, e[2]);
            g[u].push((v, w));
            g[v].push((u, w));
        }
        const INF: i64 = i64::MAX / 4;
        let mut dis = vec![vec![INF; 2]; n];
        dis[0][0] = 0;
        use std::cmp::Reverse;
        let mut q = std::collections::BinaryHeap::from([Reverse((0, 0, 0))]);
        while let Some(Reverse((cur, u, used))) = q.pop() {
            if cur > dis[u][used] {
                continue;
            }
            if u == n - 1 && used == 1 {
                return cur;
            }
            for &(v, w) in &g[u] {
                let nxt = cur + w as i64;
                if dis[v][used] > nxt {
                    dis[v][used] = nxt;
                    q.push(Reverse((nxt, v, used)));
                }
                if used == 0 {
                    let nxt = cur;
                    if dis[v][1] > nxt {
                        dis[v][1] = nxt;
                        q.push(Reverse((nxt, v, 1)));
                    }
                }
            }
        }
        dis[n - 1][1]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_matrix;
    #[test]
    pub fn test_min_cost_excluding_max_1() {
        assert_eq!(
            13,
            Solution::min_cost_excluding_max(
                5,
                lc_matrix![[0, 1, 2], [1, 2, 7], [2, 3, 7], [3, 4, 4]]
            )
        );
    }
    #[test]
    pub fn test_min_cost_excluding_max_2() {
        assert_eq!(
            0,
            Solution::min_cost_excluding_max(3, lc_matrix![[0, 1, 1], [1, 2, 1], [0, 2, 50000]])
        );
    }
}
