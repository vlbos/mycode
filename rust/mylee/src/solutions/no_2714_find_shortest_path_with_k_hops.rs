// # [2714. Find Shortest Path with K Hops](https://leetcode.com/problems/find-shortest-path-with-k-hops)

// ## Description

//  You are given a positive integer  n  which is the number of nodes of a  0-indexed undirected weighted connected  graph
// and a  0-indexed   2D array   edges  where  edges[i] = [u i , v i , w i ]  indicates that
// there is an edge between nodes  u i   and  v i   with weight  w i  .

//  You are also given two nodes  s  and  d , and a positive integer  k ,
// your task is to find the  shortest  path from  s  to  d ,
// but you can hop over  at most   k  edges.
// In other words, make the weight of  at most   k  edges  0  and then find the  shortest  path from  s  to  d .

//  Return  the length of the  shortest  path from   s   to   d   with the given condition .

//   ### Example 1:

//  Input:  n = 4, edges = [[0,1,4],[0,2,2],[2,3,6]], s = 1, d = 3, k = 2
//  Output:  2
//  Explanation:  In this example there is only one path from node 1 (the green node) to node 3 (the red node),
// which is (1->0->2->3) and the length of it is 4 + 2 + 6 = 12. Now we can make weight of two edges 0,
// we make weight of the blue edges 0, then we have 0 + 2 + 0 = 2.
// It can be shown that 2 is the minimum length of a path we can achieve with the given condition.

//  <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2700-2799/2714.Find%20Shortest%20Path%20with%20K%20Hops/images/1.jpg" style="width: 170px; height: 171px;" />

//   ### Example 2:

//  Input:  n = 7, edges = [[3,1,9],[3,2,4],[4,0,9],[0,5,6],[3,6,2],[6,0,4],[1,2,4]], s = 4, d = 1, k = 2
//  Output:  6
//  Explanation:  In this example there are 2 paths from node 4 (the green node) to node 1 (the red node),
// which are (4->0->6->3->2->1) and (4->0->6->3->1). The first one has the length 9 + 4 + 2 + 4 + 4 = 23,
// and the second one has the length 9 + 4 + 2 + 9 = 24. Now if we make weight of the blue edges 0,
// we get the shortest path with the length 0 + 4 + 2 + 0 = 6.
// It can be shown that 6 is the minimum length of a path we can achieve with the given condition.

//  <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2700-2799/2714.Find%20Shortest%20Path%20with%20K%20Hops/images/2.jpg" style="width: 400px; height: 171px;" />

//   ### Example 3:

//  Input:  n = 5, edges = [[0,4,2],[0,1,3],[0,2,1],[2,1,4],[1,3,4],[3,4,7]], s = 2, d = 3, k = 1
//  Output:  3
//  Explanation:  In this example there are 4 paths from node 2 (the green node) to node 3 (the red node),
//  which are (2->1->3), (2->0->1->3), (2->1->0->4->3) and (2->0->4->3).
// The first two have the length 4 + 4 = 1 + 3 + 4 = 8,
// the third one has the length 4 + 3 + 2 + 7 = 16 and the last one has the length 1 + 2 + 7 = 10.
// Now if we make weight of the blue edge 0, we get the shortest path with the length 1 + 2 + 0 = 3.
// It can be shown that 3 is the minimum length of a path we can achieve with the given condition.

//  <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2700-2799/2714.Find%20Shortest%20Path%20with%20K%20Hops/images/3.jpg" style="width: 300px; height: 296px;" />

//   Constraints:

// 	  2  <= n  <= 500
// 	  n - 1  <= edges.length  <= min(10^4 , n * (n - 1) / 2)
// 	  edges[i].length = 3
// 	  0  <= edges[i][0], edges[i][1]  <= n - 1
// 	  1  <= edges[i][2]  <= 10^6
// 	  0  <= s, d, k  <= n - 1
// 	  s != d
// 	 The input is generated such that the graph is  connected  and has  no repeated edges  or  self-loops

// ## Solutions
//  int shortest_path_with_hops(int n, vector<vector<int>>& edges, int s, int d,
//                            int k) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn shortest_path_with_hops(n: i32, edges: Vec<Vec<i32>>, s: i32, d: i32, k: i32) -> i32 {
        let mut g = vec![vec![]; n as usize];
        for e in &edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            g[u].push((v, e[2]));
            g[v].push((u, e[2]));
        }
        let mut dist = vec![vec![i32::MAX / 2; k as usize + 1]; n as usize];
        dist[s as usize][k as usize] = 0;
        use std::{cmp::Reverse, collections::BinaryHeap};
        let mut q = BinaryHeap::from([Reverse((dist[s as usize][k as usize], s, k))]);
        while let Some(Reverse((dis, u, hops))) = q.pop() {
            if u == d {
                return dis;
            }
            for &(v, w) in &g[u as usize] {
                if dis + w < dist[v][hops as usize] {
                    dist[v][hops as usize] = dis + w;
                    q.push(Reverse((dist[v][hops as usize], v as i32, hops)));
                }
                if hops > 0 && d < dist[v][hops as usize - 1] {
                    dist[v][hops as usize - 1] = dis;
                    q.push(Reverse((dist[v][hops as usize - 1], v as i32, hops - 1)));
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_shortest_path_with_hops_1() {
        assert_eq!(
            2,
            Solution::shortest_path_with_hops(
                4,
                vec![vec![0, 1, 4], vec![0, 2, 2], vec![2, 3, 6]],
                1,
                3,
                2
            )
        );
    }
    #[test]
    pub fn test_shortest_path_with_hops_2() {
        assert_eq!(
            6,
            Solution::shortest_path_with_hops(
                7,
                vec![
                    vec![3, 1, 9],
                    vec![3, 2, 4],
                    vec![4, 0, 9],
                    vec![0, 5, 6],
                    vec![3, 6, 2],
                    vec![6, 0, 4],
                    vec![1, 2, 4]
                ],
                4,
                1,
                2
            )
        );
    }
    #[test]
    pub fn test_shortest_path_with_hops_3() {
        assert_eq!(
            3,
            Solution::shortest_path_with_hops(
                5,
                vec![
                    vec![0, 4, 2],
                    vec![0, 1, 3],
                    vec![0, 2, 1],
                    vec![2, 1, 4],
                    vec![1, 3, 4],
                    vec![3, 4, 7]
                ],
                2,
                3,
                1
            )
        );
    }
}
