/*
 * @lc app=leetcode id=449 lang=rust
 *
 * [449] Serialize and Deserialize BST
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
struct Codec {
	
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self{}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if root.is_none(){
            return String::new();
        }
        fn post_order( root: &Option<Rc<RefCell<TreeNode>>>,str:&mut String){
                   if let Some(n)=root{
                            post_order(&n.borrow().left,str);
                            post_order(&n.borrow().right,str);
                            str.push_str(&n.borrow().val.to_string());
                            str.push(',');
                   }
        }
        let mut str=String::new();
        post_order(&root,&mut str);
        if !str.is_empty(){
            str.pop();
        }
        str
    }
	
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty(){
        return None;
        }
        let mut dv = data.split(',').collect::<Vec<&str>>();
        if dv.is_empty(){
            return None;
        }
        fn helper(lower:usize,upper:usize,dv:&mut Vec<&str>)->Option<Rc<RefCell<TreeNode>>> {
            if dv.is_empty(){
            return None;
            }
            let v = *(dv.last().unwrap());
            let val = v.parse::<i32>().unwrap();
            let vv = val as usize;
            if vv<lower || vv>upper{
                return None;
            }
            dv.pop();
            let mut ans = Rc::new(RefCell::new(TreeNode::new(val)));
            ans.borrow_mut().right=helper(vv,upper,dv);
            ans.borrow_mut().left=helper(lower,vv,dv);
            Some(ans)
        }
        helper(usize::MIN,usize::MAX,&mut dv)
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */
// @lc code=end

