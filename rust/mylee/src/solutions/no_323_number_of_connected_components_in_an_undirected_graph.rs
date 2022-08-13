// 323\. Number of Connected Components in an Undirected Graph
// ===========================================================

// Given `n` nodes labeled from `0` to `n - 1` and a list of undirected edges (each edge is a pair of nodes),
// write a function to find the number of connected components in an undirected graph.

// **Example 1:**

// **Input:** `n = 5` and `edges = [[0, 1], [1, 2], [3, 4]]`

//      0          3
//      |          |
//      1 --- 2    4

// **Output:** 2

// **Example 2:**

// **Input:** `n = 5` and `edges = [[0, 1], [1, 2], [2, 3], [3, 4]]`

//      0           4
//      |           |
//      1 --- 2 --- 3

// **Output:** 1

// **Note:**
// You can assume that no duplicate edges will appear in `edges`.
// Since all edges are undirected, `[0, 1]` is the same as `[1, 0]` and thus will not appear together in `edges`.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google) [LinkedIn](https://leetcode.ca/tags/#LinkedIn) [Microsoft](https://leetcode.ca/tags/#Microsoft) [Twitter](https://leetcode.ca/tags/#Twitter)

// @lc code=start

// pub  struct UnionFind {
//     sz: Vec<usize>,
//     id: Vec<usize>,
//     count: usize,
// }

// impl UnionFind {
//     pub fn   new(size: usize) -> UnionFind {
//         UnionFind {
//             sz: vec![0; size],
//             id: (0usize..size as usize).collect(),
//             count: size,
//         }
//     }

//     pub fn   find(&self, mut p: usize) -> usize {
//         let id = &self.id;
//         loop {
//             let q = id[p];
//             if q == p {
//                 return q;
//             } else {
//                 p = q;
//             }
//         }
//     }

//     pub fn   connected(&self, p: usize, q: usize) -> bool {
//         self.find(p) == self.find(q)
//     }

//     pub fn   union(&mut self, p: usize, q: usize) {
//         let pid = self.find(p);
//         let qid = self.find(q);
//         if pid != qid {
//             if self.sz[pid] > self.sz[qid] {
//                 self.id[qid] = pid;
//                 self.sz[pid] += self.sz[qid];
//             } else {
//                 self.id[pid] = qid;
//                 self.sz[qid] += self.sz[pid];
//             }
//             self.count -= 1;
//         }
//     }
// }

impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        // let mut uf = UnionFind::new(n as usize);
        // for e in edges {
        //     let p = e[0] as usize;
        //     let q = e[1] as usize;
        //     uf.union(p, q);
        // }
        // uf.count as i32
        let n = n as usize;
        let mut g = vec![Vec::new(); n];
        for e in edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            g[u].push(v);
            g[v].push(u);
        }
        let mut visited = vec![false; n];
        pub fn dfs(u: usize, g: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
            visited[u] = true;
            for &v in &g[u] {
                if !visited[v] {
                    dfs(v, g, visited);
                }
            }
        }
        let mut ans = 0;
        for i in 0..n {
            if !visited[i] {
                dfs(i, &g, &mut visited);
                ans += 1;
            }
        }
        ans
    }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_count_components_1() {
        assert_eq!(
            Solution::count_components(5, vec![vec![0, 1], vec![1, 2], vec![3, 4]]),
            2
        );
    }

    #[test]
    pub fn test_count_components_2() {
        assert_eq!(
            Solution::count_components(5, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]]),
            1
        );
    }
}
