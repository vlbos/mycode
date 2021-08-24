/*
 * @lc app=leetcode.cn id=700 lang=rust
 *
 * [700] 二叉搜索树中的搜索
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
    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let node = root.clone();
        if let Some(_n)=node{
                let n =_n.borrow();
                if n.val==val{
                return root;
                }else if n.val>val{
                    return Self::search_bst(n.left.clone(),val);
                }else{
                    return Self::search_bst(n.right.clone(),val);
                }
        }
        None
    }
}
// @lc code=end

