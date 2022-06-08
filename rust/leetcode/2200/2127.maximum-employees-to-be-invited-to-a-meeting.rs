/*
 * @lc app=leetcode id=2127 lang=rust
 *
 * [2127] Maximum Employees to Be Invited to a Meeting
 */

// @lc code=start
impl Solution {
    pub fn maximum_invitations(favorite: Vec<i32>) -> i32 {
         let n = favorite.len();
        let mut indeg = vec![0; n];
        for &v in &favorite {
            indeg[v as usize] += 1;
        }
        let mut used = vec![false; n];
        let mut f = vec![1; n];
        let mut q = std::collections::VecDeque::from(
            indeg
                .iter()
                .enumerate()
                .filter(|(_, &v)| v == 0)
                .map(|(i, _)| i)
                .collect::<Vec<usize>>(),
        );
        while let Some(u) = q.pop_front() {
            used[u] = true;
            let v = favorite[u as usize] as usize;
            f[v] = f[v].max(f[u] + 1);
            indeg[v] -= 1;
            if indeg[v] == 0 {
                q.push_back(v);
            }
        }
        let (mut ring, mut total) = (0, 0);
        for (i, &fav) in favorite.iter().enumerate() {
            if used[i]{
                continue;
            }
            let j = fav as usize;
            if favorite[j] == i as i32 {
                total += f[i] + f[j];
                used[i] = true;
                used[j] = true;
                continue;
            }
            let (mut u, mut cnt) = (i, 0);
            loop {
                cnt += 1;
                u = favorite[u] as usize;
                used[u] = true;
                if u == i {
                    break;
                }
            }
            ring = ring.max(cnt);
        }
        ring.max(total)
    }
}
// @lc code=end
