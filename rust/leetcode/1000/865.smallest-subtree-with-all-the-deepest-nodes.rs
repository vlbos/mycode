/*
 * @lc app=leetcode id=865 lang=rust
 *
 * [865] Smallest Subtree with all the Deepest Nodes
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
    pub fn subtree_with_all_deepest(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>)->(Option<Rc<RefCell<TreeNode>>>,i32){
            if let Some(n)=root{  
                let l = dfs(&n.borrow().left);
                let r = dfs(&n.borrow().right);
                if l.1>r.1{
                    return (l.0,l.1+1);
                }
                if l.1<r.1{
                    return (r.0,r.1+1);
                }
                return (root.clone(),l.1+1);
            }
            (None,0)
        }
        dfs(&root).0
    }
}
// @lc code=end

