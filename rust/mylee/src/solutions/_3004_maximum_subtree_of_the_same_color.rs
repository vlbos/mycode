// # [3004. Maximum Subtree of the Same Color](https://leetcode.com/problems/maximum-subtree-of-the-same-color)

// ## Description

// You are given a 2D integer array edges representing a tree with n nodes, numbered from 0 to n - 1, rooted at node 0, where edges[i] = [ui, vi] means there is an edge between the nodes vi and ui.

// You are also given a 0-indexed integer array colors of size n, where colors[i] is the color assigned to node i.

// We want to find a node v such that every node in the <span data-keyword="subtree-of-node">subtree of v has the same color.

// Return the size of such subtree with the maximum number of nodes possible.

//
// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3000-3099/3004.Maximum%20Subtree%20of%20the%20Same%20Color/images/20231216-134026.png" style="padding: 10px; background: rgb(255, 255, 255); border-radius: 0.5rem; width: 221px; height: 132px;" />

// Example 1:

// Input: edges = [[0,1],[0,2],[0,3]], colors = [1,1,2,3]
// Output: 1
// Explanation: Each color is represented as: 1 -> Red, 2 -> Green, 3 -> Blue. We can see that the subtree rooted at node 0 has children with different colors. Any other subtree is of the same color and has a size of 1. Hence, we return 1.

// Example 2:

// Input: edges = [[0,1],[0,2],[0,3]], colors = [1,1,1,1]
// Output: 4
// Explanation: The whole tree has the same color, and the subtree rooted at node 0 has the most number of nodes which is 4. Hence, we return 4.

// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3000-3099/3004.Maximum%20Subtree%20of%20the%20Same%20Color/images/20231216-134017.png" style="padding: 10px; background: rgb(255, 255, 255); border-radius: 0.5rem; width: 221px; height: 221px;" />

// Example 3:

// Input: edges = [[0,1],[0,2],[2,3],[2,4]], colors = [1,2,3,3,3]
// Output: 3
// Explanation: Each color is represented as: 1 -> Red, 2 -> Green, 3 -> Blue. We can see that the subtree rooted at node 0 has children with different colors. Any other subtree is of the same color, but the subtree rooted at node 2 has a size of 3 which is the maximum. Hence, we return 3.

//
// Constraints:

// 	n == edges.length + 1
// 	1  <= n  <= 5 * 104
// 	edges[i] == [ui, vi]
// 	0  <= ui, vi  < n
// 	colors.length == n
// 	1  <= colors[i]  <= 105
// 	The input is generated such that the graph represented by edges is a tree.

//     int maximum_subtree_size(vector<vector<int>>& edges, vector<int>& colors) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn maximum_subtree_size(edges: Vec<Vec<i32>>, colors: Vec<i32>) -> i32 {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_matrix;
    #[test]
    pub fn test_maximum_subtree_size_1() {
        assert_eq!(
            1,
            Solution::maximum_subtree_size(lc_matrix![[0, 1], [0, 2], [0, 3]], vec![1, 1, 2, 3])
        );
    }
    #[test]
    pub fn test_maximum_subtree_size_2() {
        assert_eq!(
            4,
            Solution::maximum_subtree_size(lc_matrix![[0, 1], [0, 2], [0, 3]], vec![1, 1, 1, 1])
        );
    }
    #[test]
    pub fn test_maximum_subtree_size_3() {
        assert_eq!(
            3,
            Solution::maximum_subtree_size(
                lc_matrix![[0, 1], [0, 2], [2, 3], [2, 4]],
                vec![1, 2, 3, 3, 3]
            )
        );
    }
}
