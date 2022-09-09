// # [2313. Minimum Flips in Binary Tree to Get Result](https://leetcode.com/problems/minimum-flips-in-binary-tree-to-get-result)

// ## Description

// You are given the root of a binary tree with the following properties:

//
// 	Leaf nodes have either the value 0 or 1, representing false and true respectively.
// 	Non-leaf nodes have either the value 2, 3, 4, or 5, representing the boolean operations OR, AND, XOR, and NOT, respectively.
//

// You are also given a boolean result, which is the desired result of the evaluation of the root node.

// The evaluation of a node is as follows:

//
// 	If the node is a leaf node, the evaluation is the value of the node, i.e. true or false.
// 	Otherwise, evaluate the node's children and apply the boolean operation of its value with the children's evaluations.
//

// In one operation, you can flip a leaf node, which causes a false node to become true, and a true node to become false.

// Return the minimum number of operations that need to be performed such that the evaluation of root yields result. 
// It can be shown that there is always a way to achieve result.

// A leaf node is a node that has zero children.

// Note: NOT nodes have either a left child or a right child, but other non-leaf nodes have both a left child and a right child.

// Example 1:
// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2300-2399/2313.Minimum%20Flips%20in%20Binary%20Tree%20to%20Get%20Result/images/operationstree.png" style="width: 500px; height: 179px;" />
//
// Input: root = [3,5,4,2,null,1,1,1,0], result = true
// Output: 2
// Explanation:
// It can be shown that a minimum of 2 nodes have to be flipped to make the root of the tree
// evaluate to true. One way to achieve this is shown in the diagram above.
//

// Example 2:

//
// Input: root = [0], result = false
// Output: 0
// Explanation:
// The root of the tree already evaluates to false, so 0 nodes have to be flipped.
//

// Constraints:

//
// 	The number of nodes in the tree is in the range [1, 10^5].
// 	0 <= Node.val <= 5
// 	OR, AND, and XOR nodes have 2 children.
// 	NOT nodes have 1 child.
// 	Leaf nodes have a value of 0 or 1.
// 	Non-leaf nodes have a value of 2, 3, 4, or 5.
//
//   int minimum_flips(TreeNode* root, bool result)
use super::util::tree::TreeNode;
#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
use std::hash::Hash;
use std::hash::Hasher;
// #[derive(Hash)]
#[derive(Eq, PartialEq)]
pub struct  NodeBool(Option<Rc<RefCell<TreeNode>>>,bool);
impl Hash for NodeBool {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some(v)=&self.0{
        let vv:TreeNode= v.borrow().clone();
            vv.hash(state);
        }
        self.1.hash(state);
    }
}
impl Solution {
    pub fn minimum_flips(root: Option<Rc<RefCell<TreeNode>>>,result:bool) -> i32 {
        use std::collections::HashMap;
        fn dp(root: &Option<Rc<RefCell<TreeNode>>>,target:bool,memo:&mut HashMap<NodeBool,i32>)->i32{
            if let Some(&v)=memo.get(&NodeBool(root.clone(),target)){
            return v}
            let node=root.as_ref().unwrap().borrow();
            if node.val==0||node.val==1{
            return if (node.val==1)==target{0}else{1}
            }
            if node.val==5{
                    return dp(if node.left.is_some(){&node.left}else{&node.right},!target,memo)
                }
            let next_targets= match node.val{
                2=>if target{vec![(false,true),(true,false),(true,true)]}else{vec![(false,false)]},
                3=>if target{vec![(true,true)]}else{vec![(false,true),(true,false),(false,false)]},
                _=>if target{vec![(false,true),(true,false)]}else{vec![(false,false),(true,true)]},
            };
            let mut ans=i32::MAX;
            for &(l,r) in &next_targets{
            ans=ans.min(dp(&node.left,l,memo)+dp(&node.right,r,memo));
            }
            memo.insert(NodeBool(root.clone(),target),ans);
            ans
        }
        let mut memo=HashMap::new();
        dp(&root,result,&mut memo)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tree;
    #[test]
    pub fn test_minimum_flips_1() {
        assert_eq!(
           2,
            Solution::minimum_flips(
                tree![3,5,4,2,null,1,1,1,0],true
            )
        );
    }
    #[test]
    pub fn test_minimum_flips_2() {
        assert_eq!(
            0,
            Solution::minimum_flips(
                tree![0],false
            )
        );
    }
    
}
