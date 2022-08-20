// 1548\. The Most Similar Path in a Graph
// =======================================

// We have `n` cities and `m` bi-directional `roads` where `roads[i] = [ai, bi]` connects city `ai` with city `bi`.
// Each city has a name consisting of exactly 3 upper-case English letters given in the string array `names`.
// Starting at any city `x`, you can reach any city `y` where `y != x` (i.e. the cities and the roads are forming an undirected connected graph).

// You will be given a string array `targetPath`. You should find a path in the graph of the **same length** and with the **minimum edit distance** to `targetPath`.

// You need to return _the order of the nodes in the path with the minimum edit distance_,
// The path should be of the same length of `targetPath` and should be valid (i.e. there should be a direct road between `ans[i]` and `ans[i + 1]`).
// If there are multiple answers return any one of them.

// The **edit distance** is defined as follows:

// ![](https://assets.leetcode.com/uploads/2020/08/08/edit.jpg)

// **Follow-up:** If each node can be visited only once in the path, What should you change in your solution?

// **Example 1:**

// ![](https://assets.leetcode.com/uploads/2020/08/08/e1.jpg)

// **Input:** n = 5, roads = \[\[0,2\],\[0,3\],\[1,2\],\[1,3\],\[1,4\],\[2,4\]\], names = \["ATL","PEK","LAX","DXB","HND"\], targetPath = \["ATL","DXB","HND","LAX"\]
// **Output:** \[0,2,4,2\]
// **Explanation:** \[0,2,4,2\], \[0,3,0,2\] and \[0,3,1,2\] are accepted answers.
// \[0,2,4,2\] is equivalent to \["ATL","LAX","HND","LAX"\] which has edit distance = 1 with targetPath.
// \[0,3,0,2\] is equivalent to \["ATL","DXB","ATL","LAX"\] which has edit distance = 1 with targetPath.
// \[0,3,1,2\] is equivalent to \["ATL","DXB","PEK","LAX"\] which has edit distance = 1 with targetPath.

// **Example 2:**

// ![](https://assets.leetcode.com/uploads/2020/08/08/e2.jpg)

// **Input:** n = 4, roads = \[\[1,0\],\[2,0\],\[3,0\],\[2,1\],\[3,1\],\[3,2\]\], names = \["ATL","PEK","LAX","DXB"\], targetPath = \["ABC","DEF","GHI","JKL","MNO","PQR","STU","VWX"\]
// **Output:** \[0,1,0,1,0,1,0,1\]
// **Explanation:** Any path in this graph has edit distance = 8 with targetPath.

// **Example 3:**

// **![](https://assets.leetcode.com/uploads/2020/08/09/e3.jpg)**

// **Input:** n = 6, roads = \[\[0,1\],\[1,2\],\[2,3\],\[3,4\],\[4,5\]\], names = \["ATL","PEK","LAX","ATL","DXB","HND"\], targetPath = \["ATL","DXB","HND","DXB","ATL","LAX","PEK"\]
// **Output:** \[3,4,5,4,3,2,1\]
// **Explanation:** \[3,4,5,4,3,2,1\] is the only path with edit distance = 0 with targetPath.
// It's equivalent to \["ATL","DXB","HND","DXB","ATL","LAX","PEK"\]

// **Constraints:**

// *   `2 <= n <= 100`
// *   `m == roads.length`
// *   `n - 1 <= m <= (n * (n - 1) / 2)`
// *   `0 <= ai, bi <= n - 1`
// *   `ai != bi`
// *   The graph is guaranteed to be **connected** and each pair of nodes may have **at most one** direct road.
// *   `names.length == n`
// *   `names[i].length == 3`
// *   `names[i]` consists of upper-case English letters.
// *   `1 <= targetPath.length <= 100`
// *   `targetPath[i].length == 3`
// *   `targetPath[i]` consists of upper-case English letters.

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Unknown](https://leetcode.ca/tags/#Unknown)

#[allow(dead_code)]
pub struct Solution;
impl Solution {
    pub fn most_similar(
        n: i32,
        roads: Vec<Vec<i32>>,
        names: Vec<String>,
        target_path: Vec<String>,
    ) -> Vec<i32> {
        let n = n as usize;
        let mut g = vec![Vec::new(); n];
        for r in &roads {
            let (u, v) = (r[0] as usize, r[1] as usize);
            g[u].push(v);
            g[v].push(u);
        }
        let pn = target_path.len();
        let mut dp = vec![vec![i32::MAX; n]; pn];
        let mut path1 = vec![Vec::new(); n];
        for i in 0..n {
            dp[0][i] = if names[i] == target_path[0] { 0 } else { 1 };
            path1[i].push(i as i32);
        }
        for i in 1..pn {
            let mut path2 = vec![Vec::new(); n];
            for j in 0..n {
                if dp[i - 1][j] == i32::MAX {
                    continue;
                }
                for &k in &g[j] {
                    let d = dp[i - 1][j] + if names[k] == target_path[i] { 0 } else { 1 };
                    if dp[i][k] > d {
                        dp[i][k] = d;
                        path2[k] = path1[j].clone();
                        path2[k].push(k as i32);
                    }
                }
            }
            path1 = path2;
        }
        println!("{:?}", &path1);
        let min = dp[pn - 1].iter().enumerate().min_by_key(|x| x.1).unwrap();
        path1[min.0].clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_most_similar_1() {
        assert!(std::collections::HashSet::from([
            vec![0, 2, 4, 2],
            vec![0, 3, 0, 2],
            vec![0, 3, 1, 2]
        ])
        .contains(&Solution::most_similar(
            5,
            vec![
                vec![0, 2],
                vec![0, 3],
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 4]
            ],
            ["ATL", "PEK", "LAX", "DXB", "HND"]
                .into_iter()
                .map(String::from)
                .collect::<Vec<String>>(),
            ["ATL", "DXB", "HND", "LAX"]
                .into_iter()
                .map(String::from)
                .collect::<Vec<String>>()
        )));
    }
    #[test]
    pub fn test_most_similar_2() {
        assert_eq!(
            vec![1, 0, 1, 0, 1, 0, 1, 0],
            Solution::most_similar(
                4,
                vec![
                    vec![1, 0],
                    vec![2, 0],
                    vec![3, 0],
                    vec![2, 1],
                    vec![3, 1],
                    vec![3, 2]
                ],
                ["ATL", "PEK", "LAX", "DXB"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>(),
                ["ABC", "DEF", "GHI", "JKL", "MNO", "PQR", "STU", "VWX"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>()
            )
        );
    }
    #[test]
    pub fn test_most_similar_3() {
        assert_eq!(
            vec![3, 4, 5, 4, 3, 2, 1],
            Solution::most_similar(
                6,
                vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]],
                ["ATL", "PEK", "LAX", "ATL", "DXB", "HND"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>(),
                ["ATL", "DXB", "HND", "DXB", "ATL", "LAX", "PEK"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>()
            )
        );
    }
}
