/*
 * @lc app=leetcode id=2050 lang=rust
 *
 * [2050] Parallel Courses III
 */

// @lc code=start
impl Solution {
    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut g = vec![Vec::new(); n];
        let mut deg = vec![0; n];
        for re in &relations {
            let (u, v) = (re[0] as usize-1, re[1] as usize-1);
            g[u].push(v);
            deg[v] += 1;
        }
        let mut q = std::collections::VecDeque::from(
            deg.iter()
                .enumerate()
                .filter(|x| *x.1 == 0)
                .map(|x| x.0)
                .collect::<Vec<usize>>(),
        );
        let mut f = vec![0; n];
        let mut ans = 0;
        while let Some(u) = q.pop_front() {
            f[u] += time[u];
            ans = ans.max(f[u]);
            for &v in &g[u] {
                f[v] = f[v].max(f[u]);
                deg[v] -= 1;
                if deg[v] == 0 {
                    q.push_back(v);
                }
            }
        }
        ans
    }
}
// @lc code=end
