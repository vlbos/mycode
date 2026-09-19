// [4018. Total Sum of Interaction Cost in Tree Groups II](https://leetcode.com/problems/total-sum-of-interaction-cost-in-tree-groups-ii/)
// You are given an integer `n` and an undirected tree rooted at node 0 with `n` nodes numbered from 0 to `n - 1`. The tree is represented by a 2D integer array `edges` of length `n - 1`, where `edges[i] = [ui, vi]` indicates an undirected edge between nodes `ui` and `vi`.
// You are also given an integer array `group` of length `n`, where `group[i]` denotes the group label assigned to node `i`.
// * Two nodes `u` and `v` belong to the same group if and only if `group[u] == group[v]`.
// * The **interaction cost** between two nodes is the **shortest** distance between them in the tree.
// Return the sum of interaction costs over all pairs of node indices `(u, v)` such that `0 \<= u \< v \< n` and `group[u] == group[v]`.
// The **shortest** distance between two nodes is the number of edges on the unique path connecting them in the tree.
// **Example 1:**
// **Input:** n = 3, edges = [[0,1],[1,2]], group = [1,1,1]
// **Output:** 4
// **Explanation:**
// ![](https://assets.leetcode.com/uploads/2026/05/04/screenshot-2026-05-05-at-40329am.png)
// All nodes belong to group 1. The interaction costs between the pairs of nodes are:
// * Nodes `[0, 1]`: 1
// * Nodes `[1, 2]`: 1
// * Nodes `[0, 2]`: 2
// Thus, the total interaction cost is `1 + 1 + 2 = 4`.
// **Example 2:**
// **Input:** n = 3, edges = [[0,1],[1,2]], group = [3,2,3]
// **Output:** 2
// **Explanation:**
// ![](https://assets.leetcode.com/uploads/2026/05/04/screenshot-2026-05-05-at-40416am.png)
// * Nodes 0 and 2 belong to group 3. The interaction cost between this pair is 2.
// * Node 1 belongs to a different group and forms no valid pair. Therefore, the total interaction cost is 2.
// **Example 3:**
// **Input:** n = 4, edges = [[0,1],[0,2],[0,3]], group = [1,1,4,4]
// **Output:** 3
// **Explanation:**
// ​​​​​​​​​​​​​​![](https://assets.leetcode.com/uploads/2026/05/04/screenshot-2026-05-05-at-40819am.png)
// Nodes belonging to the same groups and their interaction costs are:
// * Group 1: Nodes `[0, 1]`: 1
// * Group 4: Nodes `[2, 3]`: 2
// Thus, the total interaction cost is `1 + 2 = 3`.
// **Example 4:**
// **Input:** n = 2, edges = [[0,1]], group = [1,2]
// **Output:** 0
// **Explanation:**
// All nodes belong to different groups and there are no valid pairs. Therefore, the total interaction cost is 0.
// **Constraints:**
// * `1 \<= n \<= 105`
// * `edges.length == n - 1`
// * `edges[i] = [ui, vi]`
// * `0 \<= ui, vi \<= n - 1`
// * `group.length == n`
// * `1 \<= group[i] \<= n`
// * The input is generated such that `edges` represents a valid tree.

impl Solution {
    pub fn interaction_costs(n: i32, edges: Vec<Vec<i32>>, mut group: Vec<i32>) -> i64 {
        let n = n as usize;
        let mut degree = vec![0; n];
        for e in &edges {
            degree[e[0] as usize] += 1;
            degree[e[1] as usize] += 1;
        }
        let mut g = vec![vec![]; n];
        for i in 0..n {
            g[i] = vec![0; degree[i]];
        }
        for e in &edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            degree[u] -= 1;
            g[u][degree[u]] += v;
            degree[v] -= 1;
            g[v][degree[v]] += u;
        }

        let mut freq = vec![0; n];
        for i in 0..n {
            group[i] -= 1;
            freq[group[i] as usize] += 1;
        }
        use std::collections::HashMap;
        fn insert(
            group: i32,
            count: i64,
            freq: &[i64],
            val: &mut i64,
            map: &mut HashMap<i32, i64>,
        ) {
            *map.entry(group).or_insert(0) += count;
            let new_val = map[&group];
            let old_val = new_val - count;
            *val -= old_val * (freq[group as usize] - old_val);
            *val += new_val * (freq[group as usize] - new_val);
        }
        fn merge(
            small_val: i64,
            small_map: &HashMap<i32, i64>,
            freq: &[i64],
            val: &mut i64,
            map: &mut HashMap<i32, i64>,
        ) {
            for (k, v) in small_map {
                insert(*k, *v, freq, val, map);
            }
        }
        fn dfs(
            index: usize,
            prev: usize,
            g: &Vec<Vec<usize>>,
            group: &[i32],
            freq: &[i64],
            total: &mut i64,
        ) -> (i64, HashMap<i32, i64>) {
            let (mut current_val, mut current_map) = (0, HashMap::new());
            insert(
                group[index] as i32,
                1,
                freq,
                &mut current_val,
                &mut current_map,
            );
            for &i in &g[index] {
                if i == prev {
                    continue;
                }
                let (mut child_val, mut child_map) = dfs(i, index, g, group, freq, total);
                *total += child_val;
                if child_map.len() > current_map.len() {
                    (child_val, child_map, current_val, current_map) =
                        (current_val, current_map, child_val, child_map);
                }
                merge(
                    child_val,
                    &child_map,
                    freq,
                    &mut current_val,
                    &mut current_map,
                );
            }
            (current_val, current_map)
        }
        let mut total = 0;
        dfs(n / 2, n, &g, &group, &freq, &mut total);
        total
    }
}
#[allow(dead_code)]
pub struct Solution;
#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_matrix;
    #[test]
    pub fn test_interaction_costs_1() {
        assert_eq!(
            4,
            Solution::interaction_costs(3, lc_matrix![[0, 1], [1, 2]], vec![1, 1, 1])
        );
    }
    #[test]
    pub fn test_interaction_costs_2() {
        assert_eq!(
            2,
            Solution::interaction_costs(3, lc_matrix![[0, 1], [1, 2]], vec![3, 2, 3])
        );
    }
    #[test]
    pub fn test_interaction_costs_3() {
        assert_eq!(
            3,
            Solution::interaction_costs(4, lc_matrix![[0, 1], [0, 2], [0, 3]], vec![1, 1, 4, 4])
        );
    }
    #[test]
    pub fn test_interaction_costs_4() {
        assert_eq!(
            0,
            Solution::interaction_costs(2, lc_matrix![[0, 1]], vec![1, 2])
        );
    }
}
