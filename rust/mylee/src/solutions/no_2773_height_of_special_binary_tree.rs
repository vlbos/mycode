// # [2773. Height of Special Binary Tree](https://leetcode.com/problems/height-of-special-binary-tree)

// ## Description

// You are given a root, which is the root of a special binary tree with n nodes.
// The nodes of the special binary tree are numbered from 1 to n.
// Suppose the tree has k leaves in the following order: b1 < b2 < ... < bk.

// The leaves of this tree have a special property! That is, for every leaf bi, the following conditions hold:

//
// 	The right child of bi is bi + 1 if i < k, and b1 otherwise.
// 	The left child of bi is bi - 1 if i > 1, and bk otherwise.
//

// Return the height of the given tree.

// Note: The height of a binary tree is the length of the longest path from the root to any other node.

//
// ### Example 1:

//
// Input: root = [1,2,3,null,null,4,5]
// Output: 2
// Explanation: The given tree is shown in the following picture.
// Each leaf's left child is the leaf to its left (shown with the blue edges).
// Each leaf's right child is the leaf to its right (shown with the red edges). We can see that the graph has a height of 2.
//

// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2700-2799/2773.Height%20of%20Special%20Binary%20Tree/images/1.png" style="padding: 10px; background: rgb(255, 255, 255); border-radius: 0.5rem; width: 200px; height: 200px;" />

// ### Example 2:

//
// Input: root = [1,2]
// Output: 1
// Explanation: The given tree is shown in the following picture.
// There is only one leaf, so it doesn't have any left or right child.
// We can see that the graph has a height of 1.
//

// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2700-2799/2773.Height%20of%20Special%20Binary%20Tree/images/2.png" style="padding: 10px; background: rgb(255, 255, 255); border-radius: 0.5rem; width: 95px; height: 122px;" />

// ### Example 3:

//
// Input: root = [1,2,3,null,null,4,null,5,6]
// Output: 3
// Explanation: The given tree is shown in the following picture.
// Each leaf's left child is the leaf to its left (shown with the blue edges).
// Each leaf's right child is the leaf to its right (shown with the red edges).
//  We can see that the graph has a height of 3.
//

// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2700-2799/2773.Height%20of%20Special%20Binary%20Tree/images/3.png" style="padding: 10px; background: rgb(255, 255, 255); border-radius: 0.5rem; width: 200px; height: 280px;" />

//
// Constraints:

//
// 	n == number of nodes in the tree
// 	2 <= n <= 104
// 	1 <= node.val <= n
// 	The input is generated such that each node.val is unique.
//

// ## Solutions
// ### **C++**

// ```cpp
// /**
//  * Definition for a binary tree node.
//  * struct TreeNode {
//  *     int val;
//  *     TreeNode *left;
//  *     TreeNode *right;
//  *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
//  *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
//  *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
//  * };
//  */
// class Solution {
// public:
//     int height_of_tree(TreeNode* root) {
use super::util::tree::TreeNode;
#[allow(dead_code)]
pub struct Solution;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn height_of_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, d: i32, ans: &mut i32) {
            if root.is_none() {
                return;
            }
            *ans = d.max(*ans);
            let node = root.as_ref().unwrap().borrow();
            if node.left.is_some() && node.left.as_ref().unwrap().borrow().right != root.clone() {
                dfs(&node.left, d + 1, ans);
            }
            if node.right.is_some() && node.right.as_ref().unwrap().borrow().left != root.clone() {
                dfs(&node.right, d + 1, ans);
            }
        }
        let mut ans = 0;
        dfs(&root, 0, &mut ans);
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tree;
    #[test]
    pub fn test_height_of_tree_1() {
        assert_eq!(
            2,
            Solution::height_of_tree(tree![1, 2, 3, null, null, 4, 5])
        );
    }
    #[test]
    pub fn test_height_of_tree_2() {
        assert_eq!(1, Solution::height_of_tree(tree![1, 2]));
    }
    #[test]
    pub fn test_height_of_tree_3() {
        assert_eq!(
            3,
            Solution::height_of_tree(tree![1, 2, 3, null, null, 4, null, 5, 6])
        );
    }
}
