/*
 * @lc app=leetcode.cn id=530 lang=rust
 *
 * [530] 二叉搜索树的最小绝对差
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
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            fn dfs (root: &Option<Rc<RefCell<TreeNode>>>,pre:&mut i32,result:&mut i32){
                if let Some(_r)=root{
                    let r =_r.borrow();
                    dfs(&r.left,pre,result);
                    if *pre!=-1{
                       *result = (r.val-*pre).min(*result);
                    }
                    *pre =r.val;
                    dfs(&r.right,pre,result);
                }
            }
            let mut pre =-1;
            let mut result = i32::MAX;
            dfs(&root,&mut pre,&mut result);
            result
    }   
}
// @lc code=end

