/*
 * @lc app=leetcode id=749 lang=rust
 *
 * [749] Contain Virus
 */

// @lc code=start
impl Solution {
    pub fn contain_virus(is_infected: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashSet;
        let (m, n) = (is_infected.len() as i32, is_infected[0].len() as i32);
        let mut is_infected = is_infected;
        let mut ans = 0;
        loop {
            let mut seen = HashSet::new();
            let mut regions = Vec::new();
            let mut frontiers = Vec::new();
            let mut perimeters = Vec::new();
            for (r, row) in is_infected.iter().enumerate() {
                for (c, &v) in row.iter().enumerate() {
                    if v == 1 && !seen.contains(&(r, c)) {
                        regions.push(HashSet::new());
                        frontiers.push(HashSet::new());
                        perimeters.push(0);
                        dfs(
                            r,
                            c,
                            &is_infected,
                            &mut regions,
                            &mut frontiers,
                            &mut perimeters,
                            &mut seen,
                        );
                    }
                }
            }
            if regions.is_empty() {
                break;
            }
            let max = frontiers.iter().max_by_key(|x| x.len()).unwrap();
            let triage_index = frontiers.iter().position(|x| x.len() == max.len()).unwrap();
            ans += perimeters[triage_index];
            for (i, reg) in regions.iter().enumerate() {
                if i == triage_index {
                    for &(r, c) in reg {
                        is_infected[r][c] = -1;
                    }
                    continue;
                }
                for &(r, c) in reg {
                    for (nr, nc) in neighbors(r as i32, c as i32, m, n) {
                        if is_infected[nr][nc] == 0 {
                            is_infected[nr][nc] = 1;
                        }
                    }
                }
            }
        }
        fn neighbors(r: i32, c: i32, m: i32, n: i32) -> Vec<(usize, usize)> {
            let dirs = [0, 1, 0, -1, 0];
            let mut ans = Vec::new();
            for j in 0..dirs.len() - 1 {
                let (nr, nc) = (r + dirs[j], c + dirs[j + 1]);
                if nr < 0 || nr >= m || nc < 0 || nc >= n {
                    continue;
                }
                ans.push((nr as usize, nc as usize));
            }
            ans
        }
        fn dfs(
            r: usize,
            c: usize,
            is_infected: &Vec<Vec<i32>>,
            regions: &mut Vec<HashSet<(usize, usize)>>,
            frontiers: &mut Vec<HashSet<(usize, usize)>>,
            perimeters: &mut Vec<i32>,
            seen: &mut HashSet<(usize, usize)>,
        ) {
            let (m, n) = (is_infected.len() as i32, is_infected[0].len() as i32);
            seen.insert((r, c));
            regions.last_mut().unwrap().insert((r, c));
            for (nr, nc) in neighbors(r as i32, c as i32, m, n) {
                if is_infected[nr][nc] == 1 && !seen.contains(&(nr, nc)) {
                    dfs(nr, nc, is_infected, regions, frontiers, perimeters, seen);
                } else if is_infected[nr][nc] == 0 {
                    frontiers.last_mut().unwrap().insert((nr, nc));
                    *perimeters.last_mut().unwrap() += 1;
                }
            }
        }
        ans
    }
}
// @lc code=end
