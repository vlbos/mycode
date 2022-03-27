/*
 * @lc app=leetcode id=1457 lang=rust
 *
 * [1457] Pseudo-Palindromic Paths in a Binary Tree
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
    pub fn pseudo_palindromic_paths (root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
       fn dfs(root: &Option<Rc<RefCell<TreeNode>>>,path:&mut Vec<i32>,ans:&mut i32){
            if let Some(n)=root{
                 let node = n.borrow();
                let i = node.val as usize;
                 path[i]+=1;
                 if node.left.is_none() && node.right.is_none(){
                     if path.iter().filter(|x|*x%2>0).count()<=1{
                        *ans+=1;
                     }
                 }
                dfs(&node.left,path,ans);
                dfs(&node.right,path,ans);
                path[i]-=1;
            }
        }
        let mut path = vec![0;10];
        let mut ans = 0;
        dfs(&root,&mut path,&mut ans);
        ans
    }
}
// @lc code=end

