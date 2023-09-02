// # [2792. Count Nodes That Are Great Enough](https://leetcode.com/problems/count-nodes-that-are-great-enough)

// ## Description

// You are given a root to a binary tree and an integer k.
// A node of this tree is called great enough if the followings hold:

//
// 	Its subtree has at least k nodes.
// 	Its value is greater than the value of at least k nodes in its subtree.
//

// Return the number of nodes in this tree that are great enough.

// The node u is in the subtree of the node v,
// if  u == v or v is an ancestor of u.

//
// ### Example 1:

//
// Input: root = [7,6,5,4,3,2,1], k = 2
// Output: 3
// Explanation: Number the nodes from 1 to 7.
// The values in the subtree of node 1: {1,2,3,4,5,6,7}. Since node.val == 7,
// there are 6 nodes having a smaller value than its value. So it's great enough.
// The values in the subtree of node 2: {3,4,6}. Since node.val == 6,
// there are 2 nodes having a smaller value than its value. So it's great enough.
// The values in the subtree of node 3: {1,2,5}. Since node.val == 5,
// there are 2 nodes having a smaller value than its value. So it's great enough.
// It can be shown that other nodes are not great enough.
// See the picture below for a better understanding.

// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2700-2799/2792.Count%20Nodes%20That%20Are%20Great%20Enough/images/1.png" style="padding: 10px; background: rgb(255, 255, 255); border-radius: 0.5rem; width: 300px; height: 167px;" />

// ### Example 2:

//
// Input: root = [1,2,3], k = 1
// Output: 0
// Explanation: Number the nodes from 1 to 3.
// The values in the subtree of node 1: {1,2,3}. Since node.val == 1,
// there are no nodes having a smaller value than its value. So it's not great enough.
// The values in the subtree of node 2: {2}. Since node.val == 2,
// there are no nodes having a smaller value than its value. So it's not great enough.
// The values in the subtree of node 3: {3}. Since node.val == 3,
// there are no nodes having a smaller value than its value. So it's not great enough.
// See the picture below for a better understanding.

// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2700-2799/2792.Count%20Nodes%20That%20Are%20Great%20Enough/images/2.png" style="padding: 10px; background: rgb(255, 255, 255); border-radius: 0.5rem; width: 123px; height: 101px;" />

// ### Example 3:

//
// Input: root = [3,2,2], k = 2
// Output: 1
// Explanation: Number the nodes from 1 to 3.
// The values in the subtree of node 1: {2,2,3}. Since node.val == 3,
// there are 2 nodes having a smaller value than its value. So it's great enough.
// The values in the subtree of node 2: {2}. Since node.val == 2,
// there are no nodes having a smaller value than its value. So it's not great enough.
// The values in the subtree of node 3: {2}. Since node.val == 2,
// there are no nodes having a smaller value than its value. So it's not great enough.
// See the picture below for a better understanding.

// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2700-2799/2792.Count%20Nodes%20That%20Are%20Great%20Enough/images/3.png" style="padding: 10px; background: rgb(255, 255, 255); border-radius: 0.5rem; width: 123px; height: 101px;" />

//
// Constraints:

//
// 	The number of nodes in the tree is in the range [1, 104].
// 	1 <= Node.val <= 104
// 	1 <= k <= 10
//

// ## Solutions

// ### **C++**

// ```cpp
// /**
//  * Definition for a binary tree node.
//  * struct TreeNode {
//  *     int val;
//  *     TreeNode *left;
//  *     TreeNode *right;
//  *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
//  *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
//  *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
//  * };
//  */
// class Solution {
// public:
//     int count_great_enough_nodes(TreeNode* root, int k) {
use super::util::tree::TreeNode;
#[allow(dead_code)]
pub struct Solution;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn count_great_enough_nodes(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        use std::collections::BinaryHeap;
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, k: i32, ans: &mut i32) -> BinaryHeap<i32> {
            let push = |q: &mut BinaryHeap<i32>, x: i32| {
                q.push(x);
                if q.len() as i32 > k {
                    q.pop();
                }
            };
            if let Some(node) = root {
                let node = node.borrow();
                let (mut l, r) = (dfs(&node.left, k, ans), dfs(&node.right, k, ans));
                for &x in &r {
                    push(&mut l, x);
                }
                if l.len() as i32 == k && *l.peek().unwrap() < node.val {
                    *ans += 1;
                }
                push(&mut l, node.val);
                l
            } else {
                BinaryHeap::new()
            }
        }
        let mut ans = 0;
        dfs(&root, k, &mut ans);
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tree;
    #[test]
    pub fn test_count_great_enough_nodes_1() {
        assert_eq!(
            3,
            Solution::count_great_enough_nodes(tree![7, 6, 5, 4, 3, 2, 1], 2)
        );
    }
    #[test]
    pub fn test_count_great_enough_nodes_2() {
        assert_eq!(0, Solution::count_great_enough_nodes(tree![1, 2, 3], 1));
    }
    #[test]
    pub fn test_count_great_enough_nodes_3() {
        assert_eq!(1, Solution::count_great_enough_nodes(tree![3, 2, 2], 2));
    }
}
