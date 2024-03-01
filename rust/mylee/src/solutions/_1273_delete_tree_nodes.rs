// 1273\. Delete Tree Nodes
// ========================

// A tree rooted at node 0 is given as follows:

// *   The number of nodes is `nodes`;
// *   The value of the `i`\-th node is `value[i]`;
// *   The parent of the `i`\-th node is `parent[i]`.

// Remove every subtree whose sum of values of nodes is zero.

// After doing so, return the number of nodes remaining in the tree.

// **Example 1:**

// ![](https://assets.leetcode.com/uploads/2019/07/02/1421_sample_1.PNG)

// **Input:** nodes = 7, parent = \[-1,0,0,1,2,2,2\], value = \[1,-2,4,0,-2,-1,-1\]
// **Output:** 2

// **Constraints:**

// *   `1 <= nodes <= 10^4`
// *   `-10^5 <= value[i] <= 10^5`
// *   `parent.length == nodes`
// *   `parent[0] == -1` which indicates that `0` is the root.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Microsoft](https://leetcode.ca/tags/#Microsoft)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn delete_tree_nodes(nodes: i32, parent: Vec<i32>, value: Vec<i32>) -> i32 {
        use std::collections::{HashMap, HashSet};
        let mut t = HashMap::new();
        for (i, &p) in parent.iter().enumerate() {
            t.entry(p).or_insert(Vec::new()).push((i as i32, value[i]));
        }
        fn dfs(
            i: i32,
            value: i32,
            t: &HashMap<i32, Vec<(i32, i32)>>,
            removed: &mut HashSet<i32>,
        ) -> (HashSet<i32>, i32) {
            let mut sum = value;
            let mut child_all_nodes = HashSet::from([i]);
            for &(j, v) in t.get(&i).unwrap_or(&Vec::new()) {
                let (child_nodes, child_sum) = dfs(j, v, t, removed);
                child_all_nodes = child_all_nodes.union(&child_nodes).cloned().collect();
                sum += child_sum;
            }
            if sum == 0 {
                *removed = removed.union(&child_all_nodes).cloned().collect();
            }
            (child_all_nodes, sum)
        }
        let mut removed = HashSet::new();
        dfs(0, value[0], &t, &mut removed);
        nodes - removed.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_delete_tree_nodes_1() {
        assert_eq!(
            2,
            Solution::delete_tree_nodes(
                7,
                vec![-1, 0, 0, 1, 2, 2, 2],
                vec![1, -2, 4, 0, -2, -1, -1]
            )
        );
    }
}
