/*
 * @lc app=leetcode id=847 lang=rust
 *
 * [847] Shortest Path Visiting All Nodes
 */

// @lc code=start
impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len();
        let mut q: std::collections::VecDeque<(usize, usize, i32)> =
            (0..n).map(|x| (x, 1 << x, 0)).collect();
        let mut seen: std::collections::HashSet<(usize, usize)> =
            (0..n).map(|x| (x, 1 << x)).collect();

        let mut ans = 0;
        while let Some((u, mask, dist)) = q.pop_front() {
            if mask == (1 << n) - 1 {
                ans = dist;
                break;
            }
            for &v in &graph[u] {
                let v = v as usize;
                let v_mask = mask | (1 << v );
                if !seen.contains(&(v, v_mask)) {
                    q.push_back((v, v_mask, dist + 1));
                    seen.insert((v, v_mask));
                }
            }
        }
        ans 
    }
}
// @lc code=end
impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        let n=graph.len();
        let mut d=vec![vec![n+1;n];n];
        for i in 0..n{
            for &j in &graph[i]{
                d[i][j as usize]=1;
            }
        }
        for k in 0..n{
            for i in 0..n{
                for j in 0..n{
                    d[i][j]=d[i][j].min(d[i][k]+d[k][j]);
                }
            }
        }
        let n1=1<<n;
        let mut f=vec![vec![i32::MAX/2;n1];n];
        for mask in 1..n1{
            if mask&(mask-1)==0{
                let u=mask.trailing_zeros() as usize;
                f[u][mask]=0;
                continue
            }
            for u in 0..n{
                if mask&(1<<u)==0{
                    continue
                }
                for v in 0..n{
                    if mask&(1<<v)>0 && u!=v{
                        f[u][mask]=f[u][mask].min(f[v][mask^(1<<u)]+d[v][u] as i32);
                    }
                }
            }
        }
        (0..n).map(|u| f[u][n1-1]).min().unwrap()
    }
}