// 1245\. Tree Diameter
// ====================

// Given an undirected tree, return its diameter: the number of **edges** in a longest path in that tree.

// The tree is given as an array of `edges` where `edges[i] = [u, v]` is a bidirectional edge between nodes `u` and `v`.
//  Each node has labels in the set `{0, 1, ..., edges.length}`.

// **Example 1:**

// ![](https://assets.leetcode.com/uploads/2019/06/14/1397_example_1.PNG)

// **Input:** edges = \[\[0,1\],\[0,2\]\]
// **Output:** 2
// **Explanation:**
// A longest path of the tree is the path 1 - 0 - 2.

// **Example 2:**

// ![](https://assets.leetcode.com/uploads/2019/06/14/1397_example_2.PNG)

// **Input:** edges = \[\[0,1\],\[1,2\],\[2,3\],\[1,4\],\[4,5\]\]
// **Output:** 4
// **Explanation:**
// A longest path of the tree is the path 3 - 2 - 1 - 4 - 5.

// **Constraints:**

// *   `0 <= edges.length < 10^4`
// *   `edges[i][0] != edges[i][1]`
// *   `0 <= edges[i][j] <= edges.length`
// *   The given edges form an undirected tree.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn tree_diameter(edges: Vec<Vec<i32>>) -> i32 {
        use std::collections::{BinaryHeap, HashMap};
        let mut t = HashMap::new();
        for e in &edges {
            t.entry(e[0]).or_insert(Vec::new()).push(e[1]);
            t.entry(e[1]).or_insert(Vec::new()).push(e[0]);
        }
        fn dfs(u: i32, p: i32, t: &HashMap<i32, Vec<i32>>, ans: &mut i32) -> i32 {
            let mut q = BinaryHeap::new();
            for &v in t.get(&u).unwrap_or(&Vec::new()) {
                if v != p {
                    q.push(dfs(v, u, t, ans));
                }
            }
            if let Some(max) = q.pop() {
                *ans = (*ans).max(max + if let Some(v) = q.pop() { v } else { 0 } + 1);
                max + 1
            } else {
                0
            }
        }
        let mut ans = 0;
        for i in 0..=edges.len() {
            dfs(i as i32, -1, &t, &mut ans);
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_tree_diameter_1() {
        assert_eq!(2, Solution::tree_diameter(vec![vec![0, 1], vec![0, 2]]));
    }
    #[test]
    pub fn test_tree_diameter_2() {
        assert_eq!(
            4,
            Solution::tree_diameter(vec![
                vec![0, 1],
                vec![1, 2],
                vec![2, 3],
                vec![1, 4],
                vec![4, 5]
            ])
        );
    }
}
