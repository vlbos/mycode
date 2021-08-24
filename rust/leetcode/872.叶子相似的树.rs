/*
 * @lc app=leetcode.cn id=872 lang=rust
 *
 * [872] 叶子相似的树
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
    pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let lrd=|root: Option<Rc<RefCell<TreeNode>>>|->Vec<i32>{
                let mut s = Vec::new();
                let mut r = Vec::new();
                s.push(root);
                while let Some(n)=s.pop(){
                    if let Some(n)=n{
                        r.push(n.clone());
                        s.push(n.borrow().left.clone());
                        s.push(n.borrow().right.clone());
                       }
                }
                r.into_iter().filter(|x|x.borrow().left.is_none()&&x.borrow().right.is_none()).map(|x|x.borrow().val).collect::<Vec<i32>>()

        };
        lrd(root1)==lrd(root2)
    }
}
// @lc code=end

