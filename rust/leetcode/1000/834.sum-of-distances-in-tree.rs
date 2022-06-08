/*
 * @lc app=leetcode id=834 lang=rust
 *
 * [834] Sum of Distances in Tree
 */

// @lc code=start
impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut graph = vec![Vec::new(); n];
        for e in edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            graph[u].push(v);
            graph[v].push(u);
        }
        let mut ans = vec![0; n];
        let mut dp = vec![0; n];
        let mut sz = vec![0; n];
        fn dfs(u: usize, f: usize, graph: &Vec<Vec<usize>>, dp: &mut Vec<i32>, sz: &mut Vec<i32>) {
            dp[u] = 0;
            sz[u] = 1;
            for &v in &graph[u] {
                if v == f {
                    continue;
                }
                dfs(v, u, graph, dp, sz);
                dp[u] += dp[v] + sz[v];
                sz[u] += sz[v];
            }
        }
        fn dfs2(
            u: usize,
            f: usize,
            graph: &Vec<Vec<usize>>,
            dp: &mut Vec<i32>,
            sz: &mut Vec<i32>,
            ans: &mut Vec<i32>,
        ) {
            ans[u] = dp[u];
            for &v in &graph[u] {
                if v == f {
                    continue;
                }
                let (pu, pv) = (dp[u], dp[v]);
                let (su, sv) = (sz[u], sz[v]);
                dp[u] -= dp[v] + sz[v];
                sz[u] -= sz[v];
                dp[v] += dp[u] + sz[u];
                sz[v] += sz[u];
                dfs2(v, u, graph, dp, sz, ans);
                dp[u] = pu;
                dp[v] = pv;
                sz[u] = su;
                sz[v] = sv;
            }
        }
        dfs(0, n, &graph, &mut dp, &mut sz);
        dfs2(0, n, &graph, &mut dp, &mut sz, &mut ans);
        ans
    }
}
// @lc code=end
