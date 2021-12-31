/*
 * @lc app=leetcode id=919 lang=rust
 *
 * [919] Complete Binary Tree Inserter
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
use std::cell::RefCell;
use std::rc::Rc; 
struct CBTInserter {
nodes:Vec<Option<Rc<RefCell<TreeNode>>>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CBTInserter {

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
             let mut nodes:Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
             nodes.push(root.clone());
            let mut i = 0;
            while i<nodes.len(){
                        let left = nodes[i].as_ref().unwrap().borrow().left.clone();
                        if left.is_some() {
                            nodes.push(left);
                        }
                        let right = nodes[i].as_ref().unwrap().borrow().right.clone();

                        if right.is_some(){
                            nodes.push(right);
                        }
                 i+=1;
            }
            Self{nodes}
    }
    
    fn insert(&mut self, val: i32) -> i32 {
        let n =self.nodes.len();
        self.nodes.push(Some(Rc::new(RefCell::new(TreeNode::new(val)))));
        let mut parent =self.nodes[(n-1)>>1].as_ref().unwrap().borrow_mut();
                 if parent.left.is_none() {
                    parent.left=self.nodes[n].clone();
                 }else{
                    parent.right=self.nodes[n].clone();
                 }
                  parent.val
    }
    
    fn get_root(&self) -> Option<Rc<RefCell<TreeNode>>> {
        self.nodes.first().unwrap_or(&None).clone()
    }
}

/**
 * Your CBTInserter object will be instantiated and called as such:
 * let obj = CBTInserter::new(root);
 * let ret_1: i32 = obj.insert(val);
 * let ret_2: Option<Rc<RefCell<TreeNode>>> = obj.get_root();
 */
// @lc code=end

