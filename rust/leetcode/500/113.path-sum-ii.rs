/*
 * @lc app=leetcode id=113 lang=rust
 *
 * [113] Path Sum II
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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        fn innert_path_sum(root: &Option<Rc<RefCell<TreeNode>>>, target_sum: i32,mut seq:&mut Vec<i32>,mut ans:&mut Vec<Vec<i32>>){
            if let Some(n)=root{
                let val = n.borrow().val;
                if  val==target_sum && n.borrow().left.is_none() &&  n.borrow().right.is_none() {
                    seq.push(val);
                    ans.push(seq.clone());
                    seq.pop();
                    return;
                }
                seq.push(val);
                innert_path_sum(&n.borrow().left,target_sum-val,seq,ans);
                innert_path_sum(&n.borrow().right,target_sum-val,seq,ans);
                seq.pop();
            }

        }
        let mut ans:Vec<Vec<i32>> =Vec::new();
        let mut seq:Vec<i32> =Vec::new();
        let mut sum = 0;
        innert_path_sum(&root,target_sum,&mut seq,&mut ans);
        ans
    }
}
// @lc code=end

