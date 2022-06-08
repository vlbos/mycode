/*
 * @lc app=leetcode id=1123 lang=rust
 *
 * [1123] Lowest Common Ancestor of Deepest Leaves
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
    pub fn lca_deepest_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
       fn dfs(root: &Option<Rc<RefCell<TreeNode>>>,depth:i32,pre:&mut i32,ans:&mut  Option<Rc<RefCell<TreeNode>>>)->i32{
            if let Some(n)=root{
                let node = n.borrow();
                let l = dfs(&node.left,depth+1,pre,ans);
                let r = dfs(&node.right,depth+1,pre,ans);
                if l==r && l>=*pre{
                    *ans=Some(n.clone());
                    *pre=l;
                }
                return l.max(r);
            }
            depth
        }
        let mut ans = None;
        dfs(&root,1,&mut 0,&mut ans);
        ans
    }
}
// @lc code=end

