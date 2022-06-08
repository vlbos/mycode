/*
 * @lc app=leetcode id=1719 lang=rust
 *
 * [1719] Number Of Ways To Reconstruct A Tree
 */

// @lc code=start
impl Solution {
    pub fn check_ways(pairs: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashSet;
        let mut adj = std::collections::HashMap::new();
        for p in &pairs {
            adj.entry(p[0]).or_insert(HashSet::new()).insert(p[1]);
            adj.entry(p[1]).or_insert(HashSet::new()).insert(p[0]);
        }
        let root = *adj
            .iter()
            .find(|(k, v)| v.len() == adj.len() - 1)
            .unwrap_or((&-1, &HashSet::new()))
            .0;
        if root == -1 {
            return 0;
        }
        let mut ans = 1;
        let tmp=HashSet::new();
        for (&node, neighbours) in &adj {
            if node == root {
                continue;
            }
            let curr_degree = neighbours.len() as i32;
            let mut parent = -1;
            let mut parent_degree = i32::MAX;
            for &neighbour in neighbours {
                let nn = adj.get(&neighbour).unwrap_or(&HashSet::new()).len() as i32;
                if nn < parent_degree && nn >= curr_degree {
                    parent = neighbour;
                    parent_degree = nn;
                }
            }
            if parent == -1 {
                return 0;
            }
            let ap = adj.get(&parent).unwrap_or(&tmp);
            if neighbours
                .iter()
                .any(|neighbour| *neighbour != parent && !ap.contains(neighbour))
            {
                return 0;
            }
            if parent_degree == curr_degree {
                ans = 2;
            }
        }
        ans
    }
}
// @lc code=end
