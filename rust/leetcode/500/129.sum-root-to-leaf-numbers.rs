/*
 * @lc app=leetcode id=129 lang=rust
 *
 * [129] Sum Root to Leaf Numbers
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
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>,sum:i32,mut ans:&mut i32){
            if let Some(n)=root{
                let s = sum*10+n.borrow().val;
                if n.borrow().left.is_none() && n.borrow().right.is_none(){
                    *ans +=s;
                    return;
                }
                dfs(&n.borrow().left,s,ans);
                dfs(&n.borrow().right,s,ans);
            }
        }
        let mut ans = 0;
        dfs(&root,0,&mut ans);
        ans
    }
}
// @lc code=end

