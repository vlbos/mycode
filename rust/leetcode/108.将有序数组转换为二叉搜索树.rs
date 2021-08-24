/*
 * @lc app=leetcode.cn id=108 lang=rust
 *
 * [108] 将有序数组转换为二叉搜索树
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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
            if nums.is_empty(){
                return None;
            }
            let mid = nums.len()/2;

            let mut root = Rc::new(RefCell::new(TreeNode::new(nums[mid])));
            if mid>0{
                (&root).borrow_mut().left =  Self::sorted_array_to_bst(nums[0..mid].to_vec());
            }else{
                (&root).borrow_mut().left = None;
            }
            if mid+1<nums.len(){
                (&root).borrow_mut().right =  Self::sorted_array_to_bst(nums[mid+1..].to_vec());
            }    
            else{
                (&root).borrow_mut().right = None;
            }
            Some(root)
    }
}
// @lc code=end

