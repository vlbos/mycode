/*
 * @lc app=leetcode id=337 lang=rust
 *
 * [337] House Robber III
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
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>)->(i32,i32){
            if let Some(n)=root{
                let l = dfs(&n.borrow().left);
                let r=dfs(&n.borrow().right);
                let s = n.borrow().val+l.1+r.1;
                let o = l.0.max(l.1)+r.0.max(r.1);
                return (s,o);
            }
            (0,0)
        }
        let ans = dfs(&root);
        ans.0.max(ans.1)
    }
}
// @lc code=end

