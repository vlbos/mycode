// 742\. Closest Leaf in a Binary Tree
// ===================================

// Given a binary tree **where every node has a unique value**, and a target key `k`, find the value of the nearest leaf node to target `k` in the tree.

// Here, _nearest_ to a leaf means the least number of edges travelled on the binary tree to reach any leaf of the tree. Also, a node is called a _leaf_ if it has no children.

// In the following examples, the input tree is represented in flattened form row by row. The actual `root` tree given will be a TreeNode object.

// **Example 1:**

// **Input:**
// root = \[1, 3, 2\], k = 1
// Diagram of binary tree:
//           1
//          / \\
//         3   2

// **Output:** 2 (or 3)

// **Explanation:** Either 2 or 3 is the nearest leaf node to the target of 1.

// **Example 2:**

// **Input:**
// root = \[1\], k = 1
// **Output:** 1

// **Explanation:** The nearest leaf node is the root node itself.

// **Example 3:**

// **Input:**
// root = \[1,2,3,4,null,null,null,5,null,6\], k = 2
// Diagram of binary tree:
//              1
//             / \\
//            2   3
//           /
//          4
//         /
//        5
//       /
//      6

// **Output:** 3
// **Explanation:** The leaf node with value 3 (and not the leaf node with value 6) is nearest to the node with value 2.

// **Note:**

// 1.  `root` represents a binary tree with at least `1` node and at most `1000` nodes.
// 2.  Every node has a unique `node.val` in range `[1, 1000]`.
// 3.  There exists some node in the given binary tree for which `node.val == k`.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Databricks](https://leetcode.ca/tags/#Databricks) [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google)

use crate::solutions::util::tree::TreeNode;
// @lc code=start
use std::cell::RefCell;
// use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

impl Solution {
    pub fn find_closest_leaf(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        // let (graph, leaves) = Solution::tree_to_graph(root);
        // let mut deque = VecDeque::new();
        // deque.push_back(k);
        // let mut searched = HashSet::new();
        // while let Some(v) = deque.pop_front() {
        //     if leaves.contains(&v) {
        //         return v;
        //     }
        //     if searched.contains(&v) {
        //         continue;
        //     }
        //     searched.insert(v);
        //     for &c in &graph[&v] {
        //         deque.push_back(c);
        //     }
        // }
        // unreachable!()
        use std::collections::HashMap;
        pub fn dfs(
            root: &Option<Rc<RefCell<TreeNode>>>,
            k: i32,
            parent: &Option<Rc<RefCell<TreeNode>>>,
            parents: &mut HashMap<i32, Option<Rc<RefCell<TreeNode>>>>,
            target: &mut Option<Rc<RefCell<TreeNode>>>,
        ) {
            if root.is_none() {
                return;
            }
            if parent.is_some() {
                parents.insert(root.as_ref().unwrap().borrow().val, parent.clone());
            }
            if root.as_ref().unwrap().borrow().val == k {
                *target = root.clone();
            }
            dfs(
                &root.as_ref().unwrap().borrow().left,
                k,
                root,
                parents,
                target,
            );
            dfs(
                &root.as_ref().unwrap().borrow().right,
                k,
                root,
                parents,
                target,
            );
        }
        let mut parents = HashMap::new();
        let mut target = None;
        dfs(&root, k, &None, &mut parents, &mut target);
        if target.is_none() {
            return -1;
        }

        let mut q = std::collections::VecDeque::from([target]);
        let mut visited = std::collections::HashSet::from([k]);
        while let Some(cur) = q.pop_front() {
            let node = cur.as_ref().unwrap().borrow();
            if node.left.is_none() && node.right.is_none() {
                return node.val;
            }
            for child in [
                &node.left,
                &node.right,
                parents.get(&node.val).unwrap_or(&None),
            ] {
                if child.is_some() && !visited.contains(&child.as_ref().unwrap().borrow().val) {
                    q.push_back(child.clone());
                    visited.insert(child.as_ref().unwrap().borrow().val);
                }
            }
        }
        -1
    }

    //pub fn  tree_to_graph(
    //     root: Option<Rc<RefCell<TreeNode>>>,
    // ) -> (HashMap<i32, Vec<i32>>, HashSet<i32>) {
    //     let mut graph = HashMap::<i32, Vec<i32>>::new();
    //     let mut queue: Vec<(Option<Rc<RefCell<TreeNode>>>, Option<i32>)> = vec![(root, None)];
    //     let mut counts = HashMap::new();
    //     while let Some((n_opt, p_opt)) = queue.pop() {
    //         if let Some(n_ref) = n_opt {
    //             let n_node = n_ref.borrow();
    //             let n_val = n_node.val;
    //             counts.entry(n_val).or_insert(0);
    //             if let Some(p_val) = p_opt {
    //                 graph
    //                     .entry(n_val)
    //                     .and_modify(|v| v.push(p_val))
    //                     .or_insert_with(|| vec![p_val]);
    //                 graph
    //                     .entry(p_val)
    //                     .and_modify(|v| v.push(n_val))
    //                     .or_insert_with(|| vec![n_val]);
    //                 counts.entry(p_val).and_modify(|v| *v += 1);
    //             };
    //             queue.push((n_node.left.clone(), Some(n_val)));
    //             queue.push((n_node.right.clone(), Some(n_val)));
    //         }
    //     }
    //     (
    //         graph,
    //         counts
    //             .into_iter()
    //             .filter(|(_, v)| *v == 0)
    //             .map(|(k, _)| k)
    //             .collect(),
    //     )
    // }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;


// use std::rc::Rc;
// use std::cell::RefCell;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;

// impl Solution {
//     fn find_closest_leaf(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
//         let mut queue = VecDeque::new();
//         let mut vis = HashSet::new();
//         let mut nodes  = HashMap::new();
//         let mut edges = HashMap::new();

//         Self::preorder(&root, None, &mut nodes, &mut edges);
//         vis.insert(k);
//         queue.push_back(k);

//         while let Some(u) = queue.pop_front() {
//             if nodes[&u] {
//                 return u;
//             } else {
//                 for &v in &edges[&u] {
//                     if vis.insert(v) {
//                         queue.push_back(v);
//                     }
//                 }
//             }
//         }
//         0
//     }

//     fn preorder(root: &Option<Rc<RefCell<TreeNode>>>, parent: Option<i32>, nodes: &mut HashMap<i32, bool>, edges: &mut HashMap<i32, Vec<i32>>) {
//         if let Some(r) = root {
//             *nodes.entry(r.borrow().val).or_default() = r.borrow().left.is_none() && r.borrow().right.is_none();

//             if let Some(parent) = parent {
//                 edges.entry(parent).or_default().push(r.borrow().val);
//                 edges.entry(r.borrow().val).or_default().push(parent);
//             }

//             Self::preorder(&r.borrow().left, Some(r.borrow().val), nodes, edges);
//             Self::preorder(&r.borrow().right, Some(r.borrow().val), nodes, edges);
//         }
//     }
// }


#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;
    use std::collections::HashSet;

// [1,3,2]
// 1
// 输出
// 3
// 预期结果
// 2

    #[test]
    pub fn test_find_closest_leaf_1() {
        let tree = tree![1, 3, 2];
        let maybe = HashSet::from([2, 3]);
        assert!(maybe.contains(&Solution::find_closest_leaf(tree, 1)));
    }

    #[test]
    pub fn test_find_closest_leaf_2() {
        let tree = tree![1];
        assert_eq!(Solution::find_closest_leaf(tree, 1), 1);
    }

    #[test]
    pub fn test_find_closest_leaf_3() {
        let tree = tree![1, 2, 3, 4, null, null, null, 5, null, 6];
        assert_eq!(Solution::find_closest_leaf(tree, 2), 3);
    }
}
