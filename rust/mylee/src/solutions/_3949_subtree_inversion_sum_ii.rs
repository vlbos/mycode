// 3949. Subtree Inversion Sum II
// ## Description
// You are given an undirected tree rooted at node 0, with `n` nodes numbered from 0 to `n - 1`. The tree is represented by a 2D integer array `edges` of length `n - 1`, where `edges[i] = [ui, vi]` indicates an edge between nodes `ui` and `vi`.
// You are also given an integer array `nums` of length `n`, where `nums[i]` represents the value at node `i`, and an integer `k`.
// You may perform **inversion operations** on a subset of nodes subject to the following rules:
// * **Subtree Inversion Operation:**
// * When you invert a node, every value in the subtree rooted at that node is multiplied by -1.
// * **Distance Constraint on Inversions:**
// * You may only invert a node if it is “sufficiently far” from any other inverted node.
// * If you invert two nodes `a` and `b`, the **distance** (the number of edges on the unique path between them) must be **at least** `k`.
// Return the **maximum** possible **sum** of the tree’s node values after applying **inversion operations**.

// **Example 1:**
// **Input:** edges = [[0,1],[0,2],[0,3],[1,4],[1,5]], nums = [1,0,-10,3,4,5], k = 2
// **Output:** 23
// **Explanation:**
// [![](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3900-3999/3949.Subtree%20Inversion%20Sum%20II/images/4183example1drawio.png)](<https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3900-3999/3949.Subtree Inversion Sum II/images/4183example1drawio.png>)
// After inverting the subtree rooted at node 2, the maximum sum becomes `1 + 0 + 10 + 3 + 4 + 5 = 23`.
// **Example 2:**
// **Input:** edges = [[0,1],[1,2]], nums = [5,-10,-10], k = 1
// **Output:** 25
// **Explanation:**
// **[![](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3900-3999/3949.Subtree%20Inversion%20Sum%20II/images/4183example2drawio.png)](<https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3900-3999/3949.Subtree Inversion Sum II/images/4183example2drawio.png>)**
// After inverting the subtree rooted at node 1, the maximum sum becomes `5 + 10 + 10 = 25`.
// **Example 3:**
// **Input:** edges = [[0,1],[0,2]], nums = [1,-5,-6], k = 2
// **Output:** 12
// **Explanation:**
// [![](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3900-3999/3949.Subtree%20Inversion%20Sum%20II/images/4183example3drawio.png)](<https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3900-3999/3949.Subtree Inversion Sum II/images/4183example3drawio.png>)
// * After inverting the subtrees rooted at nodes 1 and 2, `nums = [1, 5, 6]`.
// * This is valid because nodes 1 and 2 are two edges apart (`1 → 0` and `0 → 2`), which is at least `k`.
// * The maximum sum is `1 + 5 + 6 = 12`.
// **Example 4:**
// **Input:** edges = [[0,1],[0,2]], nums = [1,-5,-6], k = 3
// **Output:** 10
// **Explanation:**
// [![](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3900-3999/3949.Subtree%20Inversion%20Sum%20II/images/4183example4drawio.png)](<https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3900-3999/3949.Subtree Inversion Sum II/images/4183example4drawio.png>)
// * After inverting the subtree rooted at nodes 0, `nums = [-1, 5, 6]`.
// * The maximum sum is `(-1) + 5 + 6 = 10`.
// * Note that we cannot invert nodes 1 and 2 because their distance is `2 \< k = 3`.

// **Constraints:**
// * `nums.length == n`
// * `edges.length == n - 1`
// * `2 \<= n \<= 5 \* 104`
// * `edges[i].length == 2`
// * `0 \<= edges[i][0], edges[i][1] \< n`
// * `-4 \* 104 \<= nums[i] \<= 4 \* 104`
// * `1 \<= k \<= 50`
// * It is guaranteed that `edges` forms a tree.

// int subtree_inversion_sum(vector<vector<int>>& edges, vector<int>& nums, int k) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn subtree_inversion_sum(edges: Vec<Vec<i32>>, nums: Vec<i32>, k: i32) -> i32 {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_matrix;
    #[test]
    pub fn test_subtree_inversion_sum_1() {
        assert_eq!(
            23,
            Solution::subtree_inversion_sum(
                lc_matrix![[0, 1], [0, 2], [0, 3], [1, 4], [1, 5]],
                vec![1, 0, -10, 3, 4, 5],
                2
            )
        );
    }
    #[test]
    pub fn test_subtree_inversion_sum_2() {
        assert_eq!(
            25,
            Solution::subtree_inversion_sum(lc_matrix![[0, 1], [1, 2]], vec![5, -10, -10], 1)
        );
    }
    #[test]
    pub fn test_subtree_inversion_sum_3() {
        assert_eq!(
            12,
            Solution::subtree_inversion_sum(lc_matrix![[0, 1], [0, 2]], vec![1, -5, -6], 2)
        );
    }
    #[test]
    pub fn test_subtree_inversion_sum_4() {
        assert_eq!(
            10,
            Solution::subtree_inversion_sum(lc_matrix![[0, 1], [0, 2]], vec![1, -5, -6], 3)
        );
    }
}
