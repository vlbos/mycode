// # [2737. Find the Closest Marked Node](https://leetcode.com/problems/find-the-closest-marked-node)

// ## Description

//  You are given a positive integer  n  which is the number of nodes of a  0-indexed directed weighted  graph
// and a  0-indexed   2D array   edges  where  edges[i] = [u i , v i , w i ]  indicates that
// there is an edge from node  u i   to node  v i   with weight  w i  .

//  You are also given a node  s  and a node array  marked ;
// your task is to find the  minimum  distance from  s  to  any  of the nodes in  marked .

//  Return  an integer denoting the minimum distance from   s   to any node in   marked
// or   -1   if there are no paths from s to any of the marked nodes .

//   ### Example 1:

//  Input:  n = 4, edges = [[0,1,1],vec![1,2,3],vec![2,3,2],vec![0,3,4]], s = 0, marked = [2,3]
//  Output:  4
//  Explanation:  There is one path from node 0 (the green node) to node 2 (a red node),
// which is 0->1->2, and has a distance of 1 + 3 = 4.
// There are two paths from node 0 to node 3 (a red node), which are 0->1->2->3 and 0->3,
// the first one has a distance of 1 + 3 + 2 = 6 and the second one has a distance of 4.
// The minimum of them is 4.

//  <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2700-2799/2737.Find%20the%20Closest%20Marked%20Node/images/image_2023-06-13_16-34-38.png" style="width: 185px; height: 180px;" />

//   ### Example 2:

//  Input:  n = 5, edges = [[0,1,2],vec![0,2,4],vec![1,3,1],vec![2,3,3],vec![3,4,2]], s = 1, marked = [0,4]
//  Output:  3
//  Explanation:  There are no paths from node 1 (the green node) to node 0 (a red node).
// There is one path from node 1 to node 4 (a red node), which is 1->3->4, and has a distance of 1 + 2 = 3.
// So the answer is 3.

//  <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2700-2799/2737.Find%20the%20Closest%20Marked%20Node/images/image_2023-06-13_16-35-13.png" style="width: 300px; height: 285px;" />

//   ### Example 3:

//  Input:  n = 4, edges = [[0,1,1],vec![1,2,3],vec![2,3,2]], s = 3, marked = [0,1]
//  Output:  -1
//  Explanation:  There are no paths from node 3 (the green node) to any of the marked nodes (the red nodes),
// so the answer is -1.

//  <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2700-2799/2737.Find%20the%20Closest%20Marked%20Node/images/image_2023-06-13_16-35-47.png" style="width: 420px; height: 80px;" />

//   Constraints:

// 	  2  <= n  <= 500
// 	  1  <= edges.length  <= 10^4
// 	  edges[i].length = 3
// 	  0  <= edges[i][0], edges[i][1]  <= n - 1
// 	  1  <= edges[i][2]  <= 10^6
// 	  1  <= marked.length  <= n - 1
// 	  0  <= s, marked[i]  <= n - 1
// 	  s != marked[i]
// 	  marked[i] != marked[j]  for every  i != j
// 	 The graph might have  repeated edges .
// 	 The graph is generated such that it has no  self-loops .

// ## Solutions

// ### **C++**

// ```cpp
// class Solution {
// public:
//     int minimum_distance(int n, vector<vector >& edges, int s, vector & marked) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn minimum_distance(n: i32, edges: Vec<Vec<i32>>, s: i32, marked: Vec<i32>) -> i32 {
        let mut g = vec![vec![i32::MAX / 2; n as usize]; n as usize];
        for e in &edges {
            g[e[0] as usize][e[1] as usize] = e[2];
        }
        let mut dist = vec![i32::MAX / 2; n as usize];
        let mut vis = vec![false; n as usize];
        dist[s as usize] = 0;
        for _ in 0..n {
            let mut t = -1;
            for j in 0..n {
                if !vis[j as usize] && (t == -1 || dist[t as usize] > dist[j as usize]) {
                    t = j;
                }
            }
            vis[t as usize] = true;
            for j in 0..n {
                dist[j as usize] =
                    dist[j as usize].min(dist[t as usize] + g[t as usize][j as usize]);
            }
        }
        let ans = marked.iter().map(|&i| dist[i as usize]).min().unwrap();
        if ans < i32::MAX / 2 {
            ans
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_minimum_distance_1() {
        assert_eq!(
            4,
            Solution::minimum_distance(
                4,
                vec![vec![0, 1, 1], vec![1, 2, 3], vec![2, 3, 2], vec![0, 3, 4]],
                0,
                vec![2, 3]
            )
        );
    }
    #[test]
    pub fn test_minimum_distance_2() {
        assert_eq!(
            3,
            Solution::minimum_distance(
                5,
                vec![
                    vec![0, 1, 2],
                    vec![0, 2, 4],
                    vec![1, 3, 1],
                    vec![2, 3, 3],
                    vec![3, 4, 2]
                ],
                1,
                vec![0, 4]
            )
        );
    }
    #[test]
    pub fn test_minimum_distance_3() {
        assert_eq!(
            -1,
            Solution::minimum_distance(
                4,
                vec![vec![0, 1, 1], vec![1, 2, 3], vec![2, 3, 2]],
                3,
                vec![0, 1]
            )
        );
    }
}
