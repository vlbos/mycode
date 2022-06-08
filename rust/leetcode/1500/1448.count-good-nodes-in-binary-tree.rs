/*
 * @lc app=leetcode id=1448 lang=rust
 *
 * [1448] Count Good Nodes in Binary Tree
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
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>,max:i32,ans:&mut i32){
            if let Some(n)=root{
                if max<=n.borrow().val{
                *ans+=1;
                }
                let m = max.max(n.borrow().val);
                dfs(&n.borrow().left,m,ans);
                dfs(&n.borrow().right,m,ans);
            }
        }
        let mut ans =0;
        dfs(&root,i32::MIN,&mut ans);
        ans
    }
}
// @lc code=end

