// # [2689. Extract Kth Character From The Rope Tree](https://leetcode.com/problems/extract-kth-character-from-the-rope-tree)

// ## Description

//  You are given the  root  of a binary tree and an integer  k .
// Besides the left and right children, every node of this tree has two other properties,
// a  string   node.val  containing only lowercase English letters (possibly empty) and a non-negative integer  node.len .
// There are two types of nodes in this tree:

// 	  Leaf : These nodes have no children,  node.len = 0 , and  node.val  is some  non-empty  string.
// 	  Internal : These nodes have at least one child (also at most two children),  node.len > 0 ,
// and  node.val  is an  empty  string.

//  The tree described above is called a  Rope  binary tree. Now we define  S[node]  recursively as follows:

// 	 If  node  is some leaf node,  S[node] = node.val ,
// 	 Otherwise if  node  is some internal node,  S[node] = concat(S[node.left], S[node.right])  and  S[node].length = node.len .

//  Return  k-th character of the string   S[root] .

//   Note:  If  s  and  p  are two strings,  concat(s, p)  is a string obtained by concatenating  p  to  s .
// For example,  concat( "ab ",  "zz ") =  "abzz " .

//   ### Example 1:

//  Input:  root = [10,4, "abcpoe ", "g ", "rta "], k = 6
//  Output:   "b "
//  Explanation:  In the picture below, we put an integer on internal nodes that represents node.len,
// and a string on leaf nodes that represents node.val.
// You can see that S[root] = concat(concat( "g ",  "rta "),  "abcpoe ") =  "grtaabcpoe ".
// So S[root][5], which represents 6th character of it, is equal to  "b ".

//  <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2600-2699/2689.Extract%20Kth%20Character%20From%20The%20Rope%20Tree/images/example1.png" style="width: 300px; height: 213px; margin-left: 280px; margin-right: 280px;" />

//   ### Example 2:

//  Input:  root = [12,6,6, "abc ", "efg ", "hij ", "klm "], k = 3
//  Output:   "c "
//  Explanation:  In the picture below, we put an integer on internal nodes that represents node.len, and a string on leaf nodes that represents node.val.
// You can see that S[root] = concat(concat( "abc ",  "efg "), concat( "hij ",  "klm ")) =  "abcefghijklm ". So S[root][2], which represents the 3rd character of it, is equal to  "c ".

//  <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2600-2699/2689.Extract%20Kth%20Character%20From%20The%20Rope%20Tree/images/example2.png" style="width: 400px; height: 232px; margin-left: 255px; margin-right: 255px;" />

//   ### Example 3:

//  Input:  root = [ "ropetree "], k = 8
//  Output:   "e "
//  Explanation:  In the picture below, we put an integer on internal nodes that represents node.len, and a string on leaf nodes that represents node.val.
// You can see that S[root] =  "ropetree ". So S[root][7], which represents 8th character of it, is equal to  "e ".

//  <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2600-2699/2689.Extract%20Kth%20Character%20From%20The%20Rope%20Tree/images/example3.png" style="width: 80px; height: 78px; margin-left: 400px; margin-right: 400px;" />

//   Constraints:

// 	 The number of nodes in the tree is in the range  [1, 10^3 ]
// 	  node.val  contains only lowercase English letters
// 	  0  <= node.val.length  <= 50
// 	  0  <= node.len  <= 10^4
// 	 for leaf nodes,  node.len = 0  and  node.val  is non-empty
// 	 for internal nodes,  node.len > 0  and  node.val  is empty
// 	  1  <= k  <= S[root].length

// ## Solutions

// ### **C++**

// ```cpp
// /**
//  * Definition for a rope tree node.
//  * struct RopeTreeNode {
//  *     int len;
//  *     string val;
//  *     RopeTreeNode *left;
//  *     RopeTreeNode *right;
//  *     RopeTreeNode() : len(0), val(""), left(nullptr), right(nullptr) {}
//  *     RopeTreeNode(string s) : len(0), val(std::move(s)), left(nullptr), right(nullptr) {}
//  *     RopeTreeNode(int x) : len(x), val(""), left(nullptr), right(nullptr) {}
//  *     RopeTreeNode(int x, RopeTreeNode *left, RopeTreeNode *right) : len(x), val(""), left(left), right(right) {}
//  * };
//  */
// class Solution {
// public:
//     char get_kth_character(RopeTreeNode* root, int k) {

// use super::util::tree::TreeNode;
// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RopeTreeNode {
    pub len: i32,
    pub val: String,
    pub left: Option<Rc<RefCell<RopeTreeNode>>>,
    pub right: Option<Rc<RefCell<RopeTreeNode>>>,
}
impl Solution {
    pub fn get_kth_character(root: Option<Rc<RefCell<RopeTreeNode>>>, k: i32) -> char {
        if let Some(node) = root {
            if node.borrow().len == 0 {
                return node.borrow().val.chars().nth(k as usize - 1).unwrap();
            }
            if let Some(left) = &node.borrow().left {
                if left.borrow().len == 0 {
                    return if k <= left.borrow().val.len() as i32 {
                        left.borrow().val.chars().nth(k as usize - 1).unwrap()
                    } else {
                        Self::get_kth_character(
                            node.borrow().right.clone(),
                            k - left.borrow().val.len() as i32,
                        )
                    };
                }
                return if left.borrow().len >= k {
                    Self::get_kth_character(node.borrow().left.clone(), k)
                } else {
                    Self::get_kth_character(node.borrow().right.clone(), k - left.borrow().len)
                };
            }
            Self::get_kth_character(node.borrow().right.clone(), k)
        } else {
            ' '
        }
    }
}
// @lc code=end
#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    // use crate::tree;

    #[test]
    pub fn test_get_kth_character_1() {
        let root = Some(Rc::new(RefCell::new(RopeTreeNode {
            len: 10,
            val: String::new(),
            left: Some(Rc::new(RefCell::new(RopeTreeNode {
                len: 4,
                val: String::new(),
                left: Some(Rc::new(RefCell::new(RopeTreeNode {
                    len: 0,
                    val: String::from("g"),
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(RopeTreeNode {
                    len: 0,
                    val: String::from("rta"),
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(RopeTreeNode {
                len: 0,
                val: String::from("abcpoe"),
                left: None,
                right: None,
            }))),
        })));
        assert_eq!(Solution::get_kth_character(root, 6), 'b');
    }
    #[test]
    pub fn test_get_kth_character_2() {
        let root = Some(Rc::new(RefCell::new(RopeTreeNode {
            len: 12,
            val: String::new(),
            left: Some(Rc::new(RefCell::new(RopeTreeNode {
                len: 6,
                val: String::new(),
                left: Some(Rc::new(RefCell::new(RopeTreeNode {
                    len: 0,
                    val: String::from("abc"),
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(RopeTreeNode {
                    len: 0,
                    val: String::from("efg"),
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(RopeTreeNode {
                len: 6,
                val: String::new(),
                left: Some(Rc::new(RefCell::new(RopeTreeNode {
                    len: 0,
                    val: String::from("hij"),
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(RopeTreeNode {
                    len: 0,
                    val: String::from("klm"),
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        assert_eq!(Solution::get_kth_character(root, 3), 'c');
    }
    #[test]
    pub fn test_get_kth_character_3() {
        let root = Some(Rc::new(RefCell::new(RopeTreeNode {
            len: 0,
            val: String::from("ropetree"),
            left: None,
            right: None,
        })));
        assert_eq!(Solution::get_kth_character(root, 8), 'e');
    }
}
