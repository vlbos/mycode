// [3313\. Find the Last Marked Nodes in Tree 🔒](https://leetcode.com/problems/find-the-last-marked-nodes-in-tree)
// ================================================================================================================

// [![](https://img.shields.io/badge/Difficulty-Hard-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Hard-4051B5?style=flat-square)

// Description
// -----------

// There exists an **undirected** tree with `n` nodes numbered `0` to `n - 1`. You are given a 2D integer array `edges` of length `n - 1`, where `edges[i] = [ui, vi]` indicates that there is an edge between nodes `ui` and `vi` in the tree.

// Initially, **all** nodes are **unmarked**. After every second, you mark all unmarked nodes which have **at least** one marked node _adjacent_ to them.

// Return an array `nodes` where `nodes[i]` is the last node to get marked in the tree, if you mark node `i` at time `t = 0`. If `nodes[i]` has _multiple_ answers for any node `i`, you can choose **any** one answer.

// **Example 1:**

// **Input:** edges = \[\[0,1\],\[0,2\]\]

// **Output:** \[2,2,1\]

// **Explanation:**

// [![](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3300-3399/3313.Find%20the%20Last%20Marked%20Nodes%20in%20Tree/images/screenshot-2024-06-02-122236.png)](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3300-3399/3313.Find%20the%20Last%20Marked%20Nodes%20in%20Tree/images/screenshot-2024-06-02-122236.png)

// *   For `i = 0`, the nodes are marked in the sequence: `[0] -> [0,1,2]`. Either 1 or 2 can be the answer.
// *   For `i = 1`, the nodes are marked in the sequence: `[1] -> [0,1] -> [0,1,2]`. Node 2 is marked last.
// *   For `i = 2`, the nodes are marked in the sequence: `[2] -> [0,2] -> [0,1,2]`. Node 1 is marked last.

// **Example 2:**

// **Input:** edges = \[\[0,1\]\]

// **Output:** \[1,0\]

// **Explanation:**

// [![](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3300-3399/3313.Find%20the%20Last%20Marked%20Nodes%20in%20Tree/images/screenshot-2024-06-02-122249.png)](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3300-3399/3313.Find%20the%20Last%20Marked%20Nodes%20in%20Tree/images/screenshot-2024-06-02-122249.png)

// *   For `i = 0`, the nodes are marked in the sequence: `[0] -> [0,1]`.
// *   For `i = 1`, the nodes are marked in the sequence: `[1] -> [0,1]`.

// **Example 3:**

// **Input:** edges = \[\[0,1\],\[0,2\],\[2,3\],\[2,4\]\]

// **Output:** \[3,3,1,1,1\]

// **Explanation:**

// [![](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3300-3399/3313.Find%20the%20Last%20Marked%20Nodes%20in%20Tree/images/screenshot-2024-06-03-210550.png)](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3300-3399/3313.Find%20the%20Last%20Marked%20Nodes%20in%20Tree/images/screenshot-2024-06-03-210550.png)

// *   For `i = 0`, the nodes are marked in the sequence: `[0] -> [0,1,2] -> [0,1,2,3,4]`.
// *   For `i = 1`, the nodes are marked in the sequence: `[1] -> [0,1] -> [0,1,2] -> [0,1,2,3,4]`.
// *   For `i = 2`, the nodes are marked in the sequence: `[2] -> [0,2,3,4] -> [0,1,2,3,4]`.
// *   For `i = 3`, the nodes are marked in the sequence: `[3] -> [2,3] -> [0,2,3,4] -> [0,1,2,3,4]`.
// *   For `i = 4`, the nodes are marked in the sequence: `[4] -> [2,4] -> [0,2,3,4] -> [0,1,2,3,4]`.

// **Constraints:**

// *   `2 <= n <= 105`
// *   `edges.length == n - 1`
// *   `edges[i].length == 2`
// *   `0 <= edges[i][0], edges[i][1] <= n - 1`
// *   The input is generated such that `edges` represents a valid tree.

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn last_marked_nodes(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len() + 1;
        let mut g = vec![vec![]; n];
        for e in edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            g[u].push(v);
            g[v].push(u);
        }
        fn dfs(u: usize, pa: usize, dist: &mut Vec<i32>, g: &Vec<Vec<usize>>) {
            for &v in &g[u] {
                if v != pa {
                    dist[v] = dist[u] + 1;
                    dfs(v, u, dist, g);
                }
            }
        }
        let mut dist1 = vec![-1; n];
        dist1[0] = 0;
        dfs(0, n, &mut dist1, &g);
        let a = dist1.iter().enumerate().max_by_key(|x| x.1).unwrap().0;

        let mut dist2 = vec![-1; n];
        dist2[a] = 0;
        dfs(a, n, &mut dist2, &g);
        let b = dist2.iter().enumerate().max_by_key(|x| x.1).unwrap().0;

        let mut dist3 = vec![-1; n];
        dist3[b] = 0;
        dfs(b, n, &mut dist3, &g);

        dist2
            .iter()
            .zip(&dist3)
            .map(|(&x, &y)| (if x > y { a } else { b }) as i32)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_matrix;
    #[test]
    pub fn test_last_marked_nodes_1() {
        let ans = Solution::last_marked_nodes(lc_matrix![[0, 1], [0, 2]]);
        assert!(vec![2, 2, 1] == ans || vec![1, 2, 1] == ans);
    }
    #[test]
    pub fn test_last_marked_nodes_2() {
        assert_eq!(vec![1, 0], Solution::last_marked_nodes(lc_matrix![[0, 1]],));
    }
    #[test]
    pub fn test_last_marked_nodes_3() {
        let ans = Solution::last_marked_nodes(lc_matrix![[0, 1], [0, 2], [2, 3], [2, 4]]);
        assert!(vec![3, 3, 1, 1, 1] == ans || vec![4, 4, 1, 1, 1] == ans);
    }
}
