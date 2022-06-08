/*
 * @lc app=leetcode id=1786 lang=rust
 *
 * [1786] Number of Restricted Paths From First to Last Node
 */

// @lc code=start
impl Solution {
    pub fn count_restricted_paths(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        use std::collections::HashMap;
        let mut q = std::collections::BinaryHeap::new();
        let mut m: HashMap<usize, HashMap<usize, i64>> = HashMap::new();
        for e in &edges {
            let (n0, n1) = (e[0] as usize, e[1] as usize);
            let e2 = e[2] as i64;
            m.entry(n0).or_insert(HashMap::new()).insert(n1, e2);
            m.entry(n1).or_insert(HashMap::new()).insert(n0, e2);
        }
        let max = 4000_000_000;
        let mut dist = vec![max; n + 1];
        dist[n] = 0;
        let mut visited = std::collections::HashSet::new();
        q.push((0, n));
        while let Some((cd, cn)) = q.pop() {
            visited.insert(cn);
            for (&next, &w) in m.get(&cn).unwrap_or(&HashMap::new()) {
                if !visited.contains(&next) {
                    if dist[cn] + w < dist[next] {
                        dist[next] = dist[cn] + w;
                        q.push((-dist[next], next));
                    }
                }
            }
        }
        let mut dp = vec![0i64; n + 1];
        dp[n] = 1i64;
        let mut a = (1..=n).collect::<Vec<usize>>();
        a.sort_by_key(|x| dist[*x]);
        for &cn in &a {
            for (&next, &w) in m.get(&cn).unwrap_or(&HashMap::new()) {
                if dist[cn] > dist[next] {
                    dp[cn] += dp[next];
                    dp[cn] %= 1000_000_007;
                }
            }
            if cn == 1 {
                break;
            }
        }
        (dp[1] % 1000_000_007) as _
    }
}
// @lc code=end
