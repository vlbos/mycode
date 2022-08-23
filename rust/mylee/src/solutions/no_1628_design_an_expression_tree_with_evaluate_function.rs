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

#[allow(dead_code)]
pub struct MyNode {
    val: String,
    left: Option<Rc<RefCell<MyNode>>>,
    right: Option<Rc<RefCell<MyNode>>>,
}
impl MyNode {
    fn new(val: String) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
    fn evaluate(&self) -> i32 {
        if let Ok(v) = self.val.parse::<i32>() {
            return v;
        }
        let (left, right) = (
            self.left.as_ref().unwrap().borrow().evaluate(),
            self.right.as_ref().unwrap().borrow().evaluate(),
        );
        match self.val.as_str() {
            "+" => left + right,
            "-" => left - right,
            "*" => left * right,
            "/" => left / right,
            _ => 0,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution;
impl Solution {
    pub fn build_tree(postfix: Vec<String>) -> i32 {
        let mut stack = Vec::new();
        for val in &postfix {
            let node = if val.parse::<i32>().is_ok() {
                MyNode::new(val.clone())
            } else {
                let (right, left) = (
                    Some(Rc::new(RefCell::new(stack.pop().unwrap()))),
                    Some(Rc::new(RefCell::new(stack.pop().unwrap()))),
                );
                MyNode {
                    val: val.clone(),
                    left,
                    right,
                }
            };
            stack.push(node);
        }
        stack.last().unwrap().evaluate()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_build_tree_1() {
        assert_eq!(
            2,
            Solution::build_tree(
                ["3", "4", "+", "2", "*", "7", "/"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>(),
            )
        );
    }
    #[test]
    pub fn test_build_tree_2() {
        assert_eq!(
            -16,
            Solution::build_tree(
                ["4", "5", "7", "2", "+", "-", "*"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>(),
            )
        );
    }
    #[test]
    pub fn test_build_tree_3() {
        assert_eq!(
            18,
            Solution::build_tree(
                ["4", "2", "+", "3", "5", "1", "-", "*", "+"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>(),
            )
        );
    }
    #[test]
    pub fn test_build_tree_4() {
        assert_eq!(
            757,
            Solution::build_tree(
                ["100", "200", "+", "2", "/", "5", "*", "7", "+"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>(),
            )
        );
    }
}
