// # [2378. Choose Edges to Maximize Score in a Tree](https://leetcode.com/problems/choose-edges-to-maximize-score-in-a-tree)

// ## Description

// You are given a weighted tree consisting of n nodes numbered from 0 to n - 1.

// The tree is rooted at node 0 and represented with a 2D array edges of size n where edges[i] = [pari, weighti] indicates that node pari is the parent of node i,
// and the edge between them has a weight equal to weighti. Since the root does not have a parent, you have edges[0] = [-1, -1].

// Choose some edges from the tree such that no two chosen edges are adjacent and the sum of the weights of the chosen edges is maximized.

// Return the maximum sum of the chosen edges.

// Note:

//
// 	You are allowed to not choose any edges in the tree, the sum of weights in this case will be 0.
// 	Two edges Edge1 and Edge2 in the tree are adjacent if they have a common node.
//
// 		In other words, they are adjacent if Edge1 connects nodes a and b and Edge2 connects nodes b and c.
//
//
//

// Example 1:
// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2300-2399/2378.Choose%20Edges%20to%20Maximize%20Score%20in%20a%20Tree/images/treedrawio.png" style="width: 271px; height: 221px;" />
//
// Input: edges = [[-1,-1],[0,5],[0,10],[2,6],[2,4]]
// Output: 11
// Explanation: The above diagram shows the edges that we have to choose colored in red.
// The total score is 5 + 6 = 11.
// It can be shown that no better score can be obtained.
//

// Example 2:
// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2300-2399/2378.Choose%20Edges%20to%20Maximize%20Score%20in%20a%20Tree/images/treee1293712983719827.png" style="width: 221px; height: 181px;" />
//
// Input: edges = [[-1,-1],[0,5],[0,-6],[0,7]]
// Output: 7
// Explanation: We choose the edge with weight 7.
// Note that we cannot choose more than one edge because all edges are adjacent to each other.
//

// Constraints:

//
// 	n == edges.length
// 	1 <= n <= 10^5
// 	edges[i].length == 2
// 	par0 == weight0 == -1
// 	0 <= pari <= n - 1 for all i >= 1.
// 	pari != i
// 	-10^6 <= weighti <= 10^6 for all i >= 1.
// 	edges represents a valid tree.
//

//  long long max_score(vector<vector<int>>& edges) {

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn max_score(edges: Vec<Vec<i32>>) -> i64 {
        let n = edges.len();
        let mut g = vec![Vec::new(); n];
        for (i, e) in edges.iter().enumerate() {
            let (p, w) = (e[0], e[1]);
            if p != -1 {
                g[p as usize].push((i, w));
            }
        }
        fn dfs(u: usize, g: &Vec<Vec<(usize, i32)>>) -> (i64, i64) {
            let (mut best_edge, mut not_take_u) = (0, 0);
            for &(v, w) in &g[u] {
                let (take_v, not_take_v) = dfs(v, g);
                best_edge = best_edge.max(w as i64 + not_take_v - take_v);
                not_take_u += take_v;
            }
            (best_edge + not_take_u, not_take_u)
        }
        let (take_root, not_take_root) = dfs(0, &g);
        take_root.max(not_take_root)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_max_score_1() {
        assert_eq!(
            11,
            Solution::max_score(vec![
                vec![-1, -1],
                vec![0, 5],
                vec![0, 10],
                vec![2, 6],
                vec![2, 4]
            ])
        );
    }
    #[test]
    pub fn test_max_score_2() {
        assert_eq!(
            7,
            Solution::max_score(vec![vec![-1, -1], vec![0, 5], vec![0, -6], vec![0, 7]])
        );
    }
}
