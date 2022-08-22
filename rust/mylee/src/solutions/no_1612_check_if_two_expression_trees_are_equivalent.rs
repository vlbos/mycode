// 1612\. Check If Two Expression Trees are Equivalent
// ===================================================

// A **[binary expression tree](https://en.wikipedia.org/wiki/Binary_expression_tree)** is a kind of binary tree used to represent arithmetic expressions.
// Each node of a binary expression tree has either zero or two children. Leaf nodes (nodes with 0 children) correspond to operands (variables),
// and internal nodes (nodes with two children) correspond to the operators. In this problem, we only consider the `'+'` operator (i.e. addition).

// You are given the roots of two binary expression trees, `root1` and `root2`. Return `true` _if the two binary expression trees are equivalent_.
// Otherwise, return `false`.

// Two binary expression trees are equivalent if they **evaluate to the same value** regardless of what the variables are set to.

// **Follow up:** What will you change in your solution if the tree also supports the `'-'` operator (i.e. subtraction)?

// **Example 1:**

// **Input:** root1 = \[x\], root2 = \[x\]
// **Output:** true

// **Example 2:**

// **![](https://assets.leetcode.com/uploads/2020/10/04/tree1.png)**

// **Input:** root1 = \[+,a,+,null,null,b,c\], root2 = \[+,+,b,c,a\]
// **Output:** true
// **Explaination:** `a + (b + c) == (b + c) + a`

// **Example 3:**

// **![](https://assets.leetcode.com/uploads/2020/10/04/tree2.png)**

// **Input:** root1 = \[+,a,+,null,null,b,c\], root2 = \[+,+,b,d,a\]
// **Output:** false
// **Explaination:** `a + (b + c) != (b + d) + a`

// **Constraints:**

// *   The number of nodes in both trees are equal, odd and, in the range `[1, 4999]`.
// *   `Node.val` is `'+'` or a lower-case English letter.
// *   It's **guaranteed** that the tree given is a valid binary expression tree.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

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
