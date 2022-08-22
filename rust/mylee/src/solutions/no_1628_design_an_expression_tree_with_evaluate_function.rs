// 1628\. Design an Expression Tree With Evaluate Function
// =======================================================

// Given the `postfix` tokens of an arithmetic expression, build and return _the binary expression tree that represents this expression._

// **Postfix** notation is a notation for writing arithmetic expressions in which the operands (numbers) appear before their operators. For example, the postfix tokens of the expression `4*(5-(2+7))` are represented in the array `postfix = ["4","5","7","2","+","-","*"]`.

// The class `Node` is an interface you should use to implement the binary expression tree. The returned tree will be tested using the `evaluate` function, which is supposed to evaluate the tree's value. You should not remove the `Node` class; however, you can modify it as you wish, and you can define other classes to implement it if needed.

// A **[binary expression tree](https://en.wikipedia.org/wiki/Binary_expression_tree)** is a kind of binary tree used to represent arithmetic expressions. Each node of a binary expression tree has either zero or two children. Leaf nodes (nodes with 0 children) correspond to operands (numbers), and internal nodes (nodes with two children) correspond to the operators `'+'` (addition), `'-'` (subtraction), `'*'` (multiplication), and `'/'` (division).

// It's guaranteed that no subtree will yield a value that exceeds `109` in absolute value, and all the operations are valid (i.e., no division by zero).

// **Follow up:** Could you design the expression tree such that it is more modular? For example, is your design able to support additional operators without making changes to your existing `evaluate` implementation?

// **Example 1:**

// **![](https://assets.leetcode.com/uploads/2020/10/15/untitled-diagram.png)**

// **Input:** s = \["3","4","+","2","\*","7","/"\]
// **Output:** 2
// **Explanation:** this expression evaluates to the above binary tree with expression (`(3+4)*2)/7) = 14/7 = 2.`

// **Example 2:**

// **![](https://assets.leetcode.com/uploads/2020/10/15/untitled-diagram2.png)**

// **Input:** s = \["4","5","7","2","+","-","\*"\]
// **Output:** -16
// **Explanation:** this expression evaluates to the above binary tree with expression 4\*(5-`(2+7)) = 4*(-4) = -16.`

// **Example 3:**

// **Input:** s = \["4","2","+","3","5","1","-","\*","+"\]
// **Output:** 18

// **Example 4:**

// **Input:** s = \["100","200","+","2","/","5","\*","7","+"\]
// **Output:** 757

// **Constraints:**

// *   `1 <= s.length < 100`
// *   `s.length` is odd.
// *   `s` consists of numbers and the characters `'+'`, `'-'`, `'*'`, and `'/'`.
// *   If `s[i]` is a number, its integer representation is no more than `105`.
// *   It is guaranteed that `s` is a valid expression.
// *   The absolute value of the result and intermediate values will not exceed `109`.
// *   It is guaranteed that no expression will include division by zero.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon)

// Node(String val, Node left, Node right) {

//     public int evaluate() {

use super::util::tree::TreeNode;

#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn check_equivalence(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        use std::collections::HashMap;
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, v: i32, freq: &mut HashMap<i32, i32>) {
            if root.is_none() {
                return;
            }
            let node = root.as_ref().unwrap().borrow();
            if (node.val as u8 as char).is_ascii_alphabetic() {
                *freq.entry(node.val).or_insert(0) += v;
            } else {
                dfs(&node.left, v, freq);
                dfs(&node.right, v, freq);
            }
        }
        let mut freq = HashMap::new();
        dfs(&root1, 1, &mut freq);
        dfs(&root2, -1, &mut freq);
        if freq.values().any(|v| *v > 0) {
            false
        } else {
            true
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    // use crate::tree;
    use super::super::util::tree::to_tree;
    fn to_exp_tree(s: &str) -> Option<Rc<RefCell<TreeNode>>> {
        to_tree(
            s.split(',')
                .map(|x| {
                    if x == "null" {
                        None
                    } else {
                        Some(x.as_bytes()[0] as i32)
                    }
                })
                .collect::<Vec<Option<i32>>>(),
        )
    }
    #[test]
    pub fn test_check_equivalence_1() {
        assert!(Solution::check_equivalence(
            to_exp_tree("x"),
            to_exp_tree("x")
        ));
    }
    #[test]
    pub fn test_check_equivalence_2() {
        assert!(Solution::check_equivalence(
            to_exp_tree("+,a,+,null,null,b,c"),
            to_exp_tree("+,+,b,c,a")
        ));
    }
    #[test]
    pub fn test_check_equivalence_3() {
        assert!(!Solution::check_equivalence(
            to_exp_tree("+,a,+,null,null,b,c"),
            to_exp_tree("+,+,b,d,a")
        ));
    }
}
