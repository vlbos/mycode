/*
 * @lc app=leetcode id=988 lang=rust
 *
 * [988] Smallest String Starting From Leaf
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
    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
       fn dfs(root: &Option<Rc<RefCell<TreeNode>>>,tmp:&mut Vec<char>,ans:&mut Vec<char>){
            if let Some(node)=root{
            tmp.push((node.borrow().val as u8+b'a') as char);
            if node.borrow().left.is_none() && node.borrow().right.is_none(){
               tmp.reverse();
               if ans.is_empty()||tmp<ans{
                    *ans = tmp.clone();
               }
               tmp.reverse();
            } 
            dfs(&node.borrow().left,tmp,ans);
            dfs(&node.borrow().right,tmp,ans);
            tmp.pop();
            }
         }
        let mut ans =Vec::new();
        let mut tmp = Vec::new();
        dfs(&root,&mut tmp,&mut ans);
        ans.iter().collect()
    }
}
// @lc code=end

