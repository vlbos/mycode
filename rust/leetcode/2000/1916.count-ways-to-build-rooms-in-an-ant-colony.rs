/*
 * @lc app=leetcode id=1916 lang=rust
 *
 * [1916] Count Ways to Build Rooms in an Ant Colony
 */

// @lc code=start
impl Solution {
    pub fn ways_to_build_rooms(prev_room: Vec<i32>) -> i32 {
        let quick_mul = |x: i64, mut y: i32| {
            let mut ans = 1;
            let mut cur = x;
            while y > 0 {
                if y & 1 > 0 {
                    ans = ans * cur % 1_000_000_007;
                }
                cur = cur * cur % 1_000_000_007;
                y >>= 1;
            }
            ans
        };
        let n = prev_room.len();
        let mut fac = vec![0; n];
        let mut inv = vec![0; n];
        fac[0]=1;
        inv[0]=1;
        for i in 1..n {
            fac[i] = fac[i - 1] * (i as i64) % 1_000_000_007;
            inv[i] = quick_mul(fac[i], 1_000_000_007 - 2);
        }
        let mut edges = vec![Vec::new(); n];
        for i in 1..n {
            edges[prev_room[i] as usize].push(i);
        }
        let mut f = vec![0; n];
        let mut cnt = vec![0; n];
        fn dfs(
            u: usize,
            edges: &Vec<Vec<usize>>,
            f: &mut Vec<i64>,
            cnt: &mut Vec<i32>,
            fac: &Vec<i64>,
            inv: &Vec<i64>,
        ) {
            f[u] = 1;
            for &v in &edges[u] {
                dfs(v, edges, f, cnt, fac, inv);
                f[u] = f[u] * f[v] % 1_000_000_007
                    * inv[cnt[v] as usize]
                    % 1_000_000_007;
                cnt[u] += cnt[v];
            }
            f[u] = f[u] * fac[cnt[u] as usize] % 1_000_000_007;
            cnt[u] += 1;
        }
        dfs(0, &edges, &mut f, &mut cnt, &fac, &inv);
        f[0] as _
    }
}
// @lc code=end
