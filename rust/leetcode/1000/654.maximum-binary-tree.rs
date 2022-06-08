/*
 * @lc app=leetcode id=654 lang=rust
 *
 * [654] Maximum Binary Tree
 */

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty(){
        return None;
        }
        let val =   *nums.iter().max().unwrap();
        let i = nums.iter().position(|x|*x==val).unwrap();
        Some(Rc::new(RefCell::new(TreeNode{val,left:Self::construct_maximum_binary_tree(nums[..i].to_vec()),right:Self::construct_maximum_binary_tree(nums[i+1..].to_vec())})))
    }
}
// @lc code=end

