
// 366\. Find Leaves of Binary Tree
// ================================

// Given a binary tree, collect a tree's nodes as if you were doing this: Collect and remove all leaves, repeat until the tree is empty.

// **Example:**

// **Input:** \[1,2,3,4,5\] 
//           1
//          / \\
//         2   3
//        / \\
//       4   5

// **Output:** \[\[4,5,3\],\[2\],\[1\]\]

// **Explanation:**

// 1\. Removing the leaves `[4,5,3]` would result in this tree:

//           1
//          /
//         2

// 2\. Now removing the leaf `[2]` would result in this tree:

//           1

// 3\. Now removing the leaf `[1]` would result in the empty tree:

//           \[\]

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Atlassian](https://leetcode.ca/tags/#Atlassian) [Google](https://leetcode.ca/tags/#Google) [LinkedIn](https://leetcode.ca/tags/#LinkedIn) [Pocket Gems](https://leetcode.ca/tags/#Pocket%20Gems)

use crate::solutions::util::tree::TreeNode;

// @lc code=start
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn   find_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    //   let mut res: Vec<Vec<i32>> = vec![];
    //   Solution::find_leaves_recursive(root, &mut res);
    //   res
        fn  dfs(root: &Option<Rc<RefCell<TreeNode>>>,ans:&mut Vec<Vec<i32>> )->i32{
            if let Some(node)=root{
                let node=node.borrow();
                let (left,right)=(dfs(&node.left,ans),dfs(&node.right,ans));
                let i = left.max(right)+1;
                if (i  as usize)<ans.len(){
                    ans[i  as usize].push(node.val);
                }else{
                    ans.push(vec![node.val]);
                }
                i 
            }else{
-1
            }
            
        }
        let mut ans=Vec::new();
        dfs(&root,&mut ans);
        ans
    }

    //pub fn  find_leaves_recursive(root: Option<Rc<RefCell<TreeNode>>>, mut res: &mut Vec<Vec<i32>>) -> i32 {
    //   match root {
    //     Some(node_ref) => {
    //       let mut node = node_ref.borrow_mut();
    //       let val = node.val;
    //       let left_leaf_distance = Solution::find_leaves_recursive(node.left.take(), &mut res);
    //       let right_leaf_distance = Solution::find_leaves_recursive(node.right.take(), &mut res);
    //       let current_leaf_distance = i32::max(left_leaf_distance, right_leaf_distance) + 1;
    //       if (res.len() as i32) <= current_leaf_distance  - 1 {
    //         res.push(vec![val]);
    //       } else {
    //         res[(current_leaf_distance  - 1) as usize].push(val);
    //       }
    //       current_leaf_distance
    //     },
    //     None => 0
    //   }
    // }
}

// @lc code=end
 
#[allow(dead_code)] 
 pub  struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::{tree_node, tree_leaf};

    #[test]
   pub fn  test_find_leaves() {
      let tree = tree_node!(
        1,
        tree_node!(2,
          tree_leaf!(4),
          tree_leaf!(5)
        ),
        tree_leaf!(3)
      );

      assert_eq!(Solution::find_leaves(tree), vec![vec![4,5,3], vec![2], vec![1]]);
    }
}