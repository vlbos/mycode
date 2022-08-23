// 1597\. Build Binary Expression Tree From Infix Expression
// =========================================================

// A **[binary expression tree](https://en.wikipedia.org/wiki/Binary_expression_tree)** is a kind of binary tree used to represent arithmetic expressions.
// Each node of a binary expression tree has either zero or two children. Leaf nodes (nodes with 0 children) correspond to operands (numbers),
//  and internal nodes (nodes with 2 children) correspond to the operators `'+'` (addition), `'-'` (subtraction), `'*'` (multiplication), and `'/'` (division).

// For each internal node with operator `o`, the [**infix expression**](https://en.wikipedia.org/wiki/Infix_notation) that it represents is `(A o B)`,
//  where `A` is the expression the left subtree represents and `B` is the expression the right subtree represents.

// You are given a string `s`, an **infix expression** containing operands, the operators described above, and parentheses `'('` and `')'`.

// Return _the **binary expression tree**, which its **[in-order traversal](https://en.wikipedia.org/wiki/Tree_traversal#In-order_(LNR))** reproduce_ `s`_._

// **Please note that order of operations applies in** `s`**.** That is, expressions in parentheses are evaluated first,
// and multiplication and division happen before addition and subtraction.

// **Example 1:**

// **![](https://assets.leetcode.com/uploads/2020/09/20/sample_1_1978.png)**

// **Input:** s = "2-3/(5\*2)+1"
// **Output:** \[+,-,1,2,/,"',' ',' ','",3,\*,"','",5,2\]

// **Example 2:**

// ![](https://assets.leetcode.com/uploads/2020/09/20/sample_2_1978.png)

// **Input:** s = "3\*4-2\*5"
// **Output:** \[-,\*,\*,3,4,2,5\]

// **Example 3:**

// **Input:** s = "1+2+3+4+5"
// **Output:** \[+,+,5,+,4,"','",+,3,"','",1,2\]

// **Constraints:**

// *   `1 <= s.length <= 105`
// *   `s` consists of digits and the characters `'+'`, `'-'`, `'*'`, `'/'`, `'('`, and `')'`.
// *   Operands in `s` are **exactly** 1 digit.
// *   It is guaranteed that `s` is a valid expression.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Unknown](https://leetcode.ca/tags/#Unknown)

use super::util::tree::TreeNode;

#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn exp_tree(s: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut nodes = Vec::new();
        let mut ops = Vec::new();
        let compare = |op1: char, op2: char| {
            if op1 == '(' || op1 == ')' {
                return false;
            }
            op1 == '*' || op1 == '/' || op2 == '+' || op2 == '-'
        };
        for c in s.chars() {
            if c.is_ascii_digit() {
                nodes.push(Some(Rc::new(RefCell::new(TreeNode::new(
                    c.to_string().parse::<i32>().unwrap(),
                )))));
            } else if c == '(' {
                ops.push(c);
            } else if c == ')' {
                while !ops.is_empty() && *ops.last().unwrap() != '(' {
                    let (op, right, left) = (
                        ops.pop().unwrap(),
                        nodes.pop().unwrap(),
                        nodes.pop().unwrap(),
                    );
                    nodes.push(Some(Rc::new(RefCell::new(TreeNode {
                        val: op as u8 as i32,
                        right,
                        left,
                    }))));
                }
                ops.pop();
            } else if "+-*/".chars().any(|x| x == c) {
                while !ops.is_empty() && compare(*ops.last().unwrap(), c) {
                    let (op, right, left) = (
                        ops.pop().unwrap(),
                        nodes.pop().unwrap(),
                        nodes.pop().unwrap(),
                    );
                    nodes.push(Some(Rc::new(RefCell::new(TreeNode {
                        val: op as u8 as i32,
                        right,
                        left,
                    }))));
                }
                ops.push(c);
            }
        }
        while !ops.is_empty() {
            let (op, right, left) = (
                ops.pop().unwrap(),
                nodes.pop().unwrap(),
                nodes.pop().unwrap(),
            );
            nodes.push(Some(Rc::new(RefCell::new(TreeNode {
                val: op as u8 as i32,
                right,
                left,
            }))));
        }

        nodes.last().unwrap().clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    // use crate::tree;
    use super::super::util::tree::to_tree;
    #[test]
    pub fn test_exp_tree_1() {
        assert_eq!(
            to_tree(
                ['+', '-', '1', '2', '/', ' ', ' ', ' ', ' ', '3', '*', ' ', ' ', '5', '2']
                    .into_iter()
                    .map(|c| if "+-*/".chars().any(|x| x == c) {
                        Some(c as u8 as i32)
                    } else {
                        c.to_string().parse::<i32>().ok()
                    })
                    .collect::<Vec<Option<i32>>>()
            ),
            Solution::exp_tree(String::from("2-3/(5*2)+1"))
        );
    }
    #[test]
    pub fn test_exp_tree_2() {
        assert_eq!(
            to_tree(
                ['-', '*', '*', '3', '4', '2', '5']
                    .into_iter()
                    .map(|c| if "+-*/".chars().any(|x| x == c) {
                        Some(c as u8 as i32)
                    } else {
                        c.to_string().parse::<i32>().ok()
                    })
                    .collect::<Vec<Option<i32>>>()
            ),
            Solution::exp_tree(String::from("3*4-2*5"))
        );
    }
    #[test]
    pub fn test_exp_tree_3() {
        assert_eq!(
            to_tree(
                ['+', '+', '5', '+', '4', ' ', ' ', '+', '3', ' ', ' ', '1', '2']
                    .into_iter()
                    .map(|c| if "+-*/".chars().any(|x| x == c) {
                        Some(c as u8 as i32)
                    } else {
                        c.to_string().parse::<i32>().ok()
                    })
                    .collect::<Vec<Option<i32>>>()
            ),
            Solution::exp_tree(String::from("1+2+3+4+5"))
        );
    }
}
