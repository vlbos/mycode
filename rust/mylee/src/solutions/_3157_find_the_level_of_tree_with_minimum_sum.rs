// # [3157. Find the Level of Tree with Minimum Sum 🔒](https://leetcode.com/problems/find-the-level-of-tree-with-minimum-sum)

// ## Description

//

// Given the root of a binary tree root where each node has a value, return the level of the tree that has the minimum sum of values among all the levels (in case of a tie, return the lowest level).

// Note that the root of the tree is at level 1 and the level of any other node is its distance from the root + 1.

//
// Example 1:

//
// Input: root = [50,6,2,30,80,7]

// Output: 2

// Explanation:

//
//

// Example 2:

//
// Input: root = [36,17,10,null,null,24]

// Output: 3

// Explanation:

//
//

// Example 3:

//
// Input: root = [5,null,5,null,5]

// Output: 1

// Explanation:

//
//

//
// Constraints:

//
// 	The number of nodes in the tree is in the range [1, 105].
// 	1 <= Node.val <= 109
//

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
//     int minimum_level(TreeNode* root) {

use super::util::tree::TreeNode;
#[allow(dead_code)]
pub struct Solution;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn minimum_level(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tree;
    #[test]
    pub fn test_minimum_level_1() {
        assert_eq!(2, Solution::minimum_level(tree![50, 6, 2, 30, 80, 7]));
    }
    #[test]
    pub fn test_minimum_level_2() {
        assert_eq!(
            3,
            Solution::minimum_level(tree![36, 17, 10, null, null, 24])
        );
    }
    #[test]
    pub fn test_minimum_level_3() {
        assert_eq!(1, Solution::minimum_level(tree![5, null, 5, null, 5]));
    }
}
