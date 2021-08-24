/*
 * @lc app=leetcode.cn id=94 lang=rust
 *
 * [94] 二叉树的中序遍历
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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
                if root.is_none(){
                    return vec![].to_vec();
                }
                // let mut l =vec![].to_vec();
                // while let Some(n) = root{
                //         if let Some(ref left) = (*n.borrow()).left {
                //             l = Solution::inorder_traversal(Some(Rc::clone(left)));
                //         }
                //         l.push((*n.borrow()).val);
                //         if let Some(ref right) = (*n.borrow()).right {
                //             l.append(&mut Solution::inorder_traversal(Some(Rc::clone(right))));
                //         }
                // }
                // l

                let mut r =Vec::<i32>::new();
                let mut s =Vec::<Option<Rc<RefCell<TreeNode>>>>::new();
                let mut p = root;
                while let Some(ref _p)=p{
                     if let n  =  Some(Rc::clone(_p)).unwrap(){
                    if let Some(ref left) = (*n.borrow()).left {
                        s.insert(0,Some(Rc::clone(&n))); 
                        p=  Some(Rc::clone(left));
                    }
                    else{
                        r.push((*n.borrow()).val);
                        if let Some(ref right) = (*n.borrow()).right {
                             p= Some(Rc::clone(right)) ;
                        }
                        else{
                                    if s.is_empty(){
                                    break;
                                    }
                            let mut q=s.remove(0);
                            let mut b = false;
                            while let Some( _q)=q{
                                 r.push((*_q.borrow()).val);    
                                 if let Some(ref right) = (*_q.borrow()).right {
                                    p= Some(Rc::clone(right)) ;
                                    b = true;
                                    break;
                                 }
                                else{
                                    if s.is_empty(){
                                        break;
                                    }
                                    q=s.remove(0);
                                }
                            }
                            if !b{
                                break;
                            }
                        }
                    }
}
                }
                r
    }
}
// @lc code=end

