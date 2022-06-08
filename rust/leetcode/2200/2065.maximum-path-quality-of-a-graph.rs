/*
 * @lc app=leetcode id=2065 lang=rust
 *
 * [2065] Maximum Path Quality of a Graph
 */

// @lc code=start
impl Solution {
    pub fn maximal_path_quality(values: Vec<i32>, edges: Vec<Vec<i32>>, max_time: i32) -> i32 {
      let n = values.len();
        let mut g = vec![Vec::new(); n];
        for e in &edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            g[u].push((v, e[2]));
            g[v].push((u, e[2]));
        }
        let mut visited = vec![false; n];
        visited[0] = true;
        let mut ans = 0;
        fn dfs(
            u: usize,
            time: i32,
            value: i32,
            values: &Vec<i32>,
            max_time: i32,
            g: &Vec<Vec<(usize, i32)>>,
            visited: &mut Vec<bool>,
            ans: &mut i32,
        ) {
            if u == 0 {
                *ans = value.max(*ans);
            }
            for &(v, dist) in &g[u] {
                if time + dist > max_time {
                    continue;
                }
                if visited[v] {
                    dfs(v, time + dist, value,values, max_time, g, visited,ans);
                } else {
                    visited[v] = true;
                    dfs(
                        v,
                        time + dist,
                        value + values[v],
                        values,
                        max_time,
                        g,
                        visited,
                        ans,
                    );
                    visited[v] = false;
                }
            }
        }
        dfs(
            0,
            0,
            values[0],
            &values,
            max_time,
            &g,
            &mut visited,
            &mut ans,
        );
        ans
    }
}
// @lc code=end
