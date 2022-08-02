// 666\. Path Sum IV
// =================

// If the depth of a tree is smaller than `5`, then this tree can be represented by a list of three-digits integers.

// For each integer in this list:

// 1.  The hundreds digit represents the depth `D` of this node, `1 <= D <= 4.`
// 2.  The tens digit represents the position `P` of this node in the level it belongs to, `1 <= P <= 8`. The position is the same as that in a full binary tree.
// 3.  The units digit represents the value `V` of this node, `0 <= V <= 9.`

// Given a list of `ascending` three-digits integers representing a binary tree with the depth smaller than 5,
// you need to return the sum of all paths from the root towards the leaves.

// **Example 1:**

// **Input:** \[113, 215, 221\]
// **Output:** 12
// **Explanation:**
// The tree that the list represents is:
//     3
//    / \\
//   5   1

// The path sum is (3 + 5) + (3 + 1) = 12.

// **Example 2:**

// **Input:** \[113, 221\]
// **Output:** 4
// **Explanation:**
// The tree that the list represents is:
//     3
//      \\
//       1

// The path sum is (3 + 1) = 4.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Alibaba](https://leetcode.ca/tags/#Alibaba)

// @lc code=start
// use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn path_sum(nums: Vec<i32>) -> i32 {
        // if nums.is_empty() {
        //     return 0;
        // }
        // let mut tree = HashMap::<(i32, i32), i32>::new();
        // let mut parents = HashSet::<(i32, i32)>::new();
        // tree.insert((-1, 0), 0);
        // for n in nums {
        //     let num = n - 110;
        //     let layer = num / 100;
        //     let index = (num - layer * 100) / 10;
        //     let value = num - layer * 100 - index * 10;
        //     let parent = (layer - 1, index / 2);
        //     let parent_value = tree[&parent];
        //     parents.insert(parent);
        //     tree.insert((layer, index), parent_value + value);
        // }
        // tree.into_iter()
        //     .filter(|(k, _)| !parents.contains(k))
        //     .map(|(_, v)| v)
        //     .fold(0, |acc, curr| acc + curr)
        if nums.is_empty() {
            return 0;
        }
        use std::collections::HashMap;
        let mut nodes = HashMap::new();
        for &num in &nums {
            nodes.insert(num / 10, num % 10);
        }
        fn dfs(root: i32, mut cur: i32, nodes: &HashMap<i32, i32>, ans: &mut i32) {
            if !nodes.contains_key(&root) {
                return;
            }
            cur += nodes.get(&root).unwrap();
            let (level, pos) = (root / 10, root % 10);
            let left = (level + 1) * 10 + 2 * pos - 1;
            let right = left + 1;
            if !nodes.contains_key(&left) && !nodes.contains_key(&right) {
                *ans += cur;
                return;
            }
            dfs(left, cur, nodes, ans);
            dfs(right, cur, nodes, ans);
        }
        let mut ans = 0;
        dfs(nums[0] / 10, 0, &nodes, &mut ans);
        ans
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_path_sum_1() {
        assert_eq!(Solution::path_sum(vec![113, 215, 221]), 12);
    }

    #[test]
    fn test_path_sum_2() {
        assert_eq!(Solution::path_sum(vec![113, 221]), 4);
    }

    #[test]
    fn test_path_sum_3() {
        assert_eq!(Solution::path_sum(vec![111, 217, 221, 315, 415]), 20);
    }
}
