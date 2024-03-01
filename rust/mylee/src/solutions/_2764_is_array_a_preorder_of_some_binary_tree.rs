// # [2764. is Array a Preorder of Some ‌Binary Tree](https://leetcode.com/problems/is-array-a-preorder-of-some-binary-tree)

// ## Description

// Given a 0-indexed integer 2D array nodes,
// your task is to determine if the given array represents the preorder traversal of some binary tree.

// For each index i, nodes[i] = [id, parentId],
// where id is the id of the node at the index i and parentId is the id of its parent in the tree (if the node has no parent,
// then parentId = -1).

// Return true if the given array represents the preorder traversal of some tree, and false otherwise.

// Note: the preorder traversal of a tree is a recursive way to traverse a tree in which we first visit the current node,
// then we do the preorder traversal for the left child, and finally, we do it for the right child.

//
// ### Example 1:

//
// Input: nodes = [[0,-1],[1,0],[2,0],[3,2],[4,2]]
// Output: true
// Explanation: The given nodes make the tree in the picture below.
// We can show that this is the preorder traversal of the tree, first we visit node 0,
// then we do the preorder traversal of the right child which is [1],
// then we do the preorder traversal of the left child which is [2,3,4].
//

// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2700-2799/2764.is%20Array%20a%20Preorder%20of%20Some%20%E2%80%8CBinary%20Tree/images/1.png" style="padding: 10px; background: #fff; border-radius: .5rem; width: 250px; height: 251px;" />

// ### Example 2:

//
// Input: nodes = [[0,-1],[1,0],[2,0],[3,1],[4,1]]
// Output: false
// Explanation: The given nodes make the tree in the picture below.
// For the preorder traversal, first we visit node 0, then we do the preorder traversal of the right child which is [1,3,4],
//  but we can see that in the given order, 2 comes between 1 and 3, so, it's not the preorder traversal of the tree.
//

// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2700-2799/2764.is%20Array%20a%20Preorder%20of%20Some%20%E2%80%8CBinary%20Tree/images/2.png" style="padding: 10px; background: #fff; border-radius: .5rem; width: 250px; height: 251px;" />

//
// Constraints:

//
// 	1 <= nodes.length <= 105
// 	nodes[i].length == 2
// 	0 <= nodes[i][0] <= 105
// 	-1 <= nodes[i][1] <= 105
// 	The input is generated such that nodes make a binary tree.
//

// ## Solutions

// **Solution 1：Depth-First Search**

// First, we construct a graph $g$ based on the $nodes$ data, where $g[i]$ represents all the child nodes of node $i$.

// Next, we design a function $dfs(i)$, which represents a pre-order traversal starting from node $i$.
// We use a variable $k$ to represent the $k$-th node in the $nodes$ list that we have currently traversed,
// with an initial value of $k = 0$.

// The execution logic of the function $dfs(i)$ is as follows:

// If $i != nodes[k][0]$, it indicates that the current sequence is not a pre-order traversal sequence of a binary tree,
// and returns false.
// Otherwise, we increment $k$ by $1$, and then recursively search all child nodes of $i$.
// If a false is found during the search, we return false immediately. Otherwise, when the search is finished, we return true.
// In the main function, we call $dfs(nodes[0][0])$. If the return value is true and $k = |nodes|$,
// then the $nodes$ sequence is a pre-order traversal sequence of a binary tree, and we return true; otherwise, we return false.

// The time complexity is $O(n)$ and the space complexity is $O(n)$, where $n$ is the number of nodes in $nodes$.

// ### **C++**

// ```cpp
// class Solution {
// public:
//     bool is_preorder(vector<vector<int>>& nodes) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn is_preorder(nodes: Vec<Vec<i32>>) -> bool {
        let mut g = vec![vec![]; nodes.len()];
        for node in &nodes {
            if node[1] != -1 {
                g[node[1] as usize].push(node[0]);
            }
        }

        fn dfs(i: i32, k: &mut i32, g: &Vec<Vec<i32>>, nodes: &Vec<Vec<i32>>) -> bool {
            if i != nodes[*k as usize][0] {
                return false;
            }
            *k += 1;
            g[i as usize].iter().all(|&j| dfs(j, k, g, nodes))
        }
        let mut k = 0;
        let root = nodes[0][0];
        dfs(root, &mut k, &g, &nodes) && k == nodes.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_is_preorder_1() {
        assert!(Solution::is_preorder(vec![
            vec![0, -1],
            vec![1, 0],
            vec![2, 0],
            vec![3, 2],
            vec![4, 2]
        ]));
    }
    #[test]
    pub fn test_is_preorder_2() {
        assert!(!Solution::is_preorder(vec![
            vec![0, -1],
            vec![1, 0],
            vec![2, 0],
            vec![3, 1],
            vec![4, 1]
        ]));
    }
}
