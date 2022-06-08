/*
 * @lc app=leetcode id=756 lang=rust
 *
 * [756] Pyramid Transition Matrix
 */

// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
        let m = 7;
        let mut t = vec![vec![0; m]; m];
        for a in &allowed {
            let ab = a.bytes().map(|x| (x - b'A') as usize).collect::<Vec<usize>>();
            t[ab[0]][ab[1]] |= (1 << (ab[2] as u32)) as usize;
        }
        let n = bottom.len();
        let mut state = vec![vec![0; n]; n];
        for (k,c) in bottom.bytes().enumerate() {
            state[n - 1][k] = (c - b'A') as usize;
        }
        let mut seen = HashSet::new();
        fn dfs(
            t: &Vec<Vec<usize>>,
            state: &mut Vec<Vec<usize>>,
            seen: &mut HashSet<usize>,
            r: usize,
            n: usize,
            i: usize,
        ) -> bool {
            if n == 1 && i == 1 {
                return true;
            }
            if i == n {
                if seen.contains(&r) {
                    return false;
                }
                seen.insert(r);
                return dfs(t, state, seen, 0, n - 1, 0);
            }

            let w = t[state[n][i]][state[n][i + 1]];
            for b in 0..7 {
                if ((w >> b) & 1) > 0 {
                    state[n - 1][i] = b;
                    if dfs(t, state, seen, r * 8 + (b + 1), n, i + 1) {
                        return true;
                    }
                }
            }
            false
        }
        dfs(&t, &mut state, &mut seen, 0, n - 1, 0)
    }
}
// @lc code=end
