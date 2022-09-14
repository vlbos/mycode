// 1430\. Check If a String Is a Valid Sequence from Root to Leaves Path in a Binary Tree
// ======================================================================================

// Given a binary tree where each path going from the root to any leaf form a **valid sequence**, check if a given string is a **valid sequence** in such binary tree.

// We get the given string from the concatenation of an array of integers `arr` and the concatenation of all values of the nodes along a path results in a **sequence** in the given binary tree.

// **Example 1:**

// **![](https://assets.leetcode.com/uploads/2019/12/18/leetcode_testcase_1.png)**

// **Input:** root = \[0,1,0,0,1,0,null,null,1,0,0\], arr = \[0,1,0,1\]
// **Output:** true
// **Explanation:** The path 0 -> 1 -> 0 -> 1 is a valid sequence (green color in the figure).
// Other valid sequences are:
// 0 -> 1 -> 1 -> 0
// 0 -> 0 -> 0

// **Example 2:**

// **![](https://assets.leetcode.com/uploads/2019/12/18/leetcode_testcase_2.png)**

// **Input:** root = \[0,1,0,0,1,0,null,null,1,0,0\], arr = \[0,0,1\]
// **Output:** false
// **Explanation:** The path 0 -> 0 -> 1 does not exist, therefore it is not even a sequence.

// **Example 3:**

// **![](https://assets.leetcode.com/uploads/2019/12/18/leetcode_testcase_3.png)**

// **Input:** root = \[0,1,0,0,1,0,null,null,1,0,0\], arr = \[0,1,1\]
// **Output:** false
// **Explanation:** The path 0 -> 1 -> 1 is a sequence, but it is not a valid sequence.

// **Constraints:**

// *   `1 <= arr.length <= 5000`
// *   `0 <= arr[i] <= 9`
// *   Each node's value is between \[0 - 9\].

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [23&me](https://leetcode.ca/tags/#23&me)
use super::util::tree::TreeNode;

#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_valid_sequence(root: Option<Rc<RefCell<TreeNode>>>, arr: Vec<i32>) -> bool {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, arr: &Vec<i32>, i: usize) -> bool {
            if root.is_none()||arr[i] !=  root.as_ref().unwrap().borrow().val {
                return  false ;
            }
            let node =root.as_ref().unwrap().borrow();
            if i == arr.len() - 1  {
                    if  node.left.is_none() && node.right.is_none()
                {true}else{false}
            }else{
                dfs(&node.left, arr, i + 1) || dfs(&node.right, arr, i + 1)
            }
            
        }
        dfs(&root, &arr, 0)
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::tree;
// [8,3,null,2,1,5,4]
// [8]
// 输出：
// true
// 预期结果：
// false
    #[test]
    pub fn test_is_valid_sequence_1() {
        assert!(Solution::is_valid_sequence(
            tree![0, 1, 0, 0, 1, 0, null, null, 1, 0, 0],
            vec![0, 1, 0, 1]
        ));
    }
    #[test]
    pub fn test_is_valid_sequence_2() {
        assert!(!Solution::is_valid_sequence(
            tree![0, 1, 0, 0, 1, 0, null, null, 1, 0, 0],
            vec![0, 0, 1]
        ));
    }
    #[test]
    pub fn test_is_valid_sequence_3() {
        assert!(!Solution::is_valid_sequence(
            tree![0, 1, 0, 0, 1, 0, null, null, 1, 0, 0],
            vec![0, 1, 1]
        ));
    }
}
