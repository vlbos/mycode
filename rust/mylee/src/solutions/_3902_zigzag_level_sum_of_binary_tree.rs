// 3902. Zigzag Level Sum of Binary Tree
// ## Description
// You are given the `root` of a **binary tree**.
// Traverse the tree level by level using a zigzag pattern:
// * At **odd**-numbered levels (1-indexed), traverse nodes from **left to right**.
// * At **even**-numbered levels, traverse nodes from **right to left**.
// While traversing a level in the specified direction, process nodes in order and **stop** immediately before the first node that violates the condition:
// * At **odd** levels: the node does not have a **left** child.
// * At **even** levels: the node does not have a **right** child.
// Only the nodes processed before this stopping condition contribute to the level sum.
// Return an integer array `ans` where `ans[i]` is the **sum** of the node values that are processed at level `i + 1`.

// **Example 1:**
// **Input:** root = [5,2,8,1,null,9,6]
// **Output:** [5,8,0]
// **Explanation:**
// [![](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3900-3999/3902.Zigzag%20Level%20Sum%20of%20Binary%20Tree/images/screenshot-2026-04-13-at-22054am.png)](<https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3900-3999/3902.Zigzag Level Sum of Binary Tree/images/screenshot-2026-04-13-at-22054am.png>)​​​​​​​
// * At level 1, nodes are processed left to right. Node 5 is included, thus `ans[0] = 5`.
// * At level 2, nodes are processed right to left. Node 8 is included, but node 2 lacks a right child, so processing stops, thus `ans[1] = 8`.
// * At level 3, nodes are processed left to right. The first node 1 lacks a left child, so no nodes are included, and `ans[2] = 0`.
// * Thus, `ans = [5, 8, 0]`.
// **Example 2:**
// **Input:** root = [1,2,3,4,5,null,7]
// **Output:** [1,5,0]
// **Explanation:**
// [![](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3900-3999/3902.Zigzag%20Level%20Sum%20of%20Binary%20Tree/images/screenshot-2026-04-13-at-22232am.png)](<https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3900-3999/3902.Zigzag Level Sum of Binary Tree/images/screenshot-2026-04-13-at-22232am.png>)
// * At level 1, nodes are processed left to right. Node 1 is included, thus `ans[0] = 1`.
// * At level 2, nodes are processed right to left. Nodes 3 and 2 are included since both have right children, thus `ans[1] = 3 + 2 = 5`.
// * At level 3, nodes are processed left to right. The first node 4 lacks a left child, so no nodes are included, and `ans[2] = 0`.
// * Thus, `ans = [1, 5, 0]`.

// **Constraints:**
// * The number of nodes in the tree is in the range `[1, 105]`.
// * `-105 \<= Node.val \<= 105`

// vector<long long> zigzag_level_sum(TreeNode* root) {

use crate::solutions::util::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn zigzag_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        vec![]
    }
}


#[allow(dead_code)]
pub struct Solution;

impl Solution {}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tree;
    #[test]
    pub fn test_zigzag_level_sum_1() {
        assert_eq!(Solution::zigzag_level_sum(tree![5,2,8,1,null,9,6]), vec![5,8,0]);
    }

    #[test]
    pub fn test_zigzag_level_sum_2() {
        assert_eq!(
            Solution::zigzag_level_sum(tree![1,2,3,4,5,null,7]),
            vec![1,5,0]
        );
    }
}
