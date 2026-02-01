// [3807\. Minimum Cost to Repair Edges to Traverse a Graph 🔒](https://leetcode.com/problems/minimum-cost-to-repair-edges-to-traverse-a-graph)
// ============================================================================================================================================

// [![](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)

// Description
// -----------

// You are given an **undirected graph** with `n` nodes labeled from 0 to `n - 1`. The graph consists of `m` edges represented by a 2D integer array `edges`, where `edges[i] = [ui, vi, wi]` indicates that there is an edge between nodes `ui` and `vi` with a repair cost of `wi`.

// You are also given an integer `k`. Initially, **all** edges are damaged.

// You may choose a non-negative integer `money` and repair **all** edges whose repair cost is **less than or equal** to `money`. All other edges remain damaged and cannot be used.

// You want to travel from node 0 to node `n - 1` using at most `k` edges.

// Return an integer denoting the **minimum** amount of money required to make this possible, or return -1 if it is impossible.

// **Example 1:**

// **[![](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3800-3899/3807.Minimum%20Cost%20to%20Repair%20Edges%20to%20Traverse%20a%20Graph/images/ex1drawio.png)](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3800-3899/3807.Minimum%20Cost%20to%20Repair%20Edges%20to%20Traverse%20a%20Graph/images/ex1drawio.png)**

// **Input:** n = 3, edges = \[\[0,1,10\],\[1,2,10\],\[0,2,100\]\], k = 1

// **Output:** 100

// **Explanation:**

// The only valid path using at most `k = 1` edge is `0 -> 2`, which requires repairing the edge with cost 100. Therefore, the minimum required amount of money is 100.

// **Example 2:**

// **[![](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3800-3899/3807.Minimum%20Cost%20to%20Repair%20Edges%20to%20Traverse%20a%20Graph/images/ex2drawio.png)](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3800-3899/3807.Minimum%20Cost%20to%20Repair%20Edges%20to%20Traverse%20a%20Graph/images/ex2drawio.png)**

// **Input:** n = 6, edges = \[\[0,2,5\],\[2,3,6\],\[3,4,7\],\[4,5,5\],\[0,1,10\],\[1,5,12\],\[0,3,9\],\[1,2,8\],\[2,4,11\]\], k = 2

// **Output:** 12

// **Explanation:**

// *   With `money = 12`, all edges with repair cost at most 12 become usable.
// *   This allows the path `0 -> 1 -> 5`, which uses exactly 2 edges and reaches node 5.
// *   If `money < 12`, there is no available path of length at most `k = 2` from node 0 to node 5.
// *   Therefore, the minimum required money is 12.

// **Example 3:**

// **[![](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3800-3899/3807.Minimum%20Cost%20to%20Repair%20Edges%20to%20Traverse%20a%20Graph/images/ex3drawio.png)](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3800-3899/3807.Minimum%20Cost%20to%20Repair%20Edges%20to%20Traverse%20a%20Graph/images/ex3drawio.png)​​​​​​​**

// **Input:** n = 3, edges = \[\[0,1,1\]\], k = 1

// **Output:** \-1

// **Explanation:**

// It is impossible to reach node 2 from node 0 using any amount of money. Therefore, the answer is -1.

// **Constraints:**

// *   `2 <= n <= 5 * 104`
// *   `1 <= edges.length == m <= 105`
// *   `edges[i] = [ui, vi, wi]`
// *   `0 <= ui, vi < n`
// *   `1 <= wi <= 109`
// *   `1 <= k <= n`
// *   There are no self-loops or duplicate edges in the graph.

//   int min_cost(int n, vector<vector<int>>& edges, int k) {

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn min_cost(n: i32, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_matrix;
    #[test]
    pub fn test_min_cost_1() {
        assert_eq!(
            100,
            Solution::min_cost(3, lc_matrix![[0, 1, 10], [1, 2, 10], [0, 2, 100]], 1)
        );
    }
    #[test]
    pub fn test_min_cost_2() {
        assert_eq!(
            12,
            Solution::min_cost(
                6,
                lc_matrix![
                    [0, 2, 5],
                    [2, 3, 6],
                    [3, 4, 7],
                    [4, 5, 5],
                    [0, 1, 10],
                    [1, 5, 12],
                    [0, 3, 9],
                    [1, 2, 8],
                    [2, 4, 11]
                ],
                2
            )
        );
    }
    #[test]
    pub fn test_min_cost_3() {
        assert_eq!(-1, Solution::min_cost(3, lc_matrix![[0, 1, 1]], 1));
    }
}
