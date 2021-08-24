/*
 * @lc app=leetcode.cn id=606 lang=rust
 *
 * [606] 根据二叉树创建字符串
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
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
          if let Some(_n)=root{
                let n = _n.borrow();
                let mut ls ="".to_string();
                let mut rs ="".to_string();
                return match (&n.left,&n.right){
                 (Some(l),Some(r))=>{
                    ls = Self::tree2str(n.left.clone());
                    rs = Self::tree2str(n.right.clone());
                    format!("{}({})({})",n.val,ls,rs)
                },
                (Some(l),None)=>{
                    ls = Self::tree2str(n.left.clone());
                    format!("{}({})",n.val,ls)
                },
                (None,Some(r))=>{
                    rs = Self::tree2str(n.right.clone());
                    format!("{}()({})",n.val,rs)
                },
                _ =>format!("{}",n.val),
                };
          }
          "".to_string()
    }
}
// @lc code=end

