// 261\. Graph Valid Tree
// ======================

// Given `n` nodes labeled from `0` to `n-1` and a list of undirected edges (each edge is a pair of nodes), write a function to check whether these edges make up a valid tree.

// **Example 1:**

// **Input:** `n = 5`, and `edges = [[0,1], [0,2], [0,3], [1,4]]`
// **Output:** true

// **Example 2:**

// **Input:** `n = 5,` and `edges = [[0,1], [1,2], [2,3], [1,3], [1,4]]`
// **Output:** false

// **Note**: you can assume that no duplicate edges will appear in `edges`. Since all edges are undirected, `[0,1]` is the same as `[1,0]` and thus will not appear together in `edges`.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Adobe](https://leetcode.ca/tags/#Adobe) [Amazon](https://leetcode.ca/tags/#Amazon) [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google) [LinkedIn](https://leetcode.ca/tags/#LinkedIn) [Pinterest](https://leetcode.ca/tags/#Pinterest) [Salesforce](https://leetcode.ca/tags/#Salesforce) [Zenefits](https://leetcode.ca/tags/#Zenefits)

// @lc code=start
// pub  struct  UnionFind {
//     sz: Vec<usize>,
//     id: Vec<usize>,
//     count: usize,
// }

// impl UnionFind {
//     pub fn   new(size: usize) -> Self {
//         UnionFind {
//             id: (vec![0; size]).iter().enumerate().map(|(i, _)| i).collect(),
//             sz: vec![1; size],
//             count: size,
//         }
//     }

//     pub fn   union(&mut self, p: usize, q: usize) {
//         let pid = self.find(p);
//         let qid = self.find(q);
//         if pid == qid {
//             return;
//         }
//         if self.sz[pid] > self.sz[qid] {
//             self.id[qid] = pid;
//             self.sz[pid] += self.sz[qid];
//         } else {
//             self.id[pid] = qid;
//             self.sz[qid] = self.sz[pid];
//         }
//         self.count -= 1;
//     }

//     pub fn   find(&self, mut p: usize) -> usize {
//         loop {
//             let pid = self.id[p];
//             if pid == p {
//                 return p;
//             }
//             p = pid;
//         }
//     }

//     pub fn   connected(&self, p: usize, q: usize) -> bool {
//         self.find(p) == self.find(q)
//     }
// }

impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        // if n < 0 {
        //     return false;
        // } else if n == 0 {
        //     return true;
        // }
        // let mut uf = UnionFind::new(n as usize);
        // for edge in edges {
        //     let s = edge[0] as usize;
        //     let e = edge[1] as usize;
        //     if uf.connected(s, e) {
        //         return false;
        //     }
        //     uf.union(s, e);
        // }
        // uf.count == 1
        let n = n as usize;
        let mut g = vec![Vec::new(); n];
        for e in &edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            g[u].push(v);
            g[v].push(u);
        }
        use std::collections::HashSet;
        let mut visited = HashSet::new();
        pub fn dfs(u: usize, g: &Vec<Vec<usize>>, visited: &mut HashSet<usize>) {
            visited.insert(u);
            for &v in &g[u] {
                if !visited.contains(&v) {
                    dfs(v, g, visited);
                }
            }
        }
        dfs(0, &g, &mut visited);
        visited.len() == n && edges.len() == n - 1
    }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_valid_tree_1() {
        let edges = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4]];
        assert!(Solution::valid_tree(5, edges));
    }

    #[test]
    pub fn test_valid_tree_2() {
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![1, 3], vec![1, 4]];
        assert!(!Solution::valid_tree(5, edges));
    }
}
