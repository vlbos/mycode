/*
 * @lc app=leetcode id=1315 lang=rust
 *
 * [1315] Sum of Nodes with Even-Valued Grandparent
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
    pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>,parent:bool,grand_parent:bool,ans:&mut i32){
            if let Some(n)=root{
                if grand_parent{    
                    *ans+=n.borrow().val;    
                }
                dfs(&n.borrow().left,n.borrow().val%2==0,parent,ans);
                dfs(&n.borrow().right,n.borrow().val%2==0,parent,ans);   
            }
        }
        let mut ans = 0;
        dfs(&root,false,false,&mut ans);
        ans
    }
}
// @lc code=end

