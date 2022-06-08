/*
 * @lc app=leetcode id=864 lang=rust
 *
 * [864] Shortest Path to Get All Keys
 */

// @lc code=start
impl Solution {
    pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
       let (m, n) = (grid.len(), grid[0].len());
        use std::collections::HashMap;
        let location: HashMap<char, (usize, usize)> = grid.clone()
            .into_iter()
            .enumerate()
            .map(|(r, row)| {
                row.chars()
                    .enumerate()
                    .filter(|(c, v)| *v != '.' && *v != '#')
                    .map(|(c, v)| (v, (r, c)))
                    .collect::<Vec<(char, (usize, usize))>>()
            })
            .flatten()
            .collect();
        let neighbors = |r: i32, c: i32| -> Vec<(usize, usize)> {
            let dirs = [0, 1, 0, -1, 0];
            let mut ans = Vec::new();
            for i in 0..dirs.len() - 1 {
                let (nr, nc) = (r + dirs[i], c + dirs[i + 1]);
                if nr < 0 || nr >= m as i32 || nc < 0 || nc >= n as i32 {
                    continue;
                }
                ans.push((nr as usize, nc as usize));
            }
            ans
        };
        let bfs_from = |source: char| -> HashMap<char, i32> {
            let (r, c) = *location.get(&source).unwrap();
            let mut seen = vec![vec![false; n]; m];
            seen[r][c] = true;
            let mut q = std::collections::VecDeque::from([(r, c, 0)]);
            let mut dist = HashMap::new();
            while let Some((r, c, d)) = q.pop_front() {
                let v = grid[r].chars().nth(c).unwrap();
                if source != v && v != '.' {
                    dist.insert(v, d);
                    continue;
                }
                for (cr, cc) in neighbors(r as i32, c as i32) {
                    if grid[cr].chars().nth(cc).unwrap() != '#' && !seen[cr][cc] {
                        seen[cr][cc] = true;
                        q.push_back((cr, cc, d + 1));
                    }
                }
            }
            dist
        };
        let dists: HashMap<char, HashMap<char, i32>> = location
            .keys()
            .map(|place| (*place, bfs_from(*place)))
            .collect();
        let target_state = (1 << (location.len() / 2)) - 1;
        use std::cmp::Reverse;
        let mut pq = std::collections::BinaryHeap::from([Reverse((0, '@', 0))]);
        let mut final_dist = HashMap::from([(('@', 0), 0)]);
        while let Some(Reverse((d, place, state))) = pq.pop() {
            if *final_dist.get(&(place, state)).unwrap_or(&i32::MAX) < d {
                continue;
            }
            if state == target_state {
                return d;
            }
            for (&destination, &d2) in dists.get(&place).unwrap_or(&HashMap::new()) {
                let mut state2 = state;
                if destination.is_ascii_lowercase() {
                    state2 |= 1 << ((destination as u8 - b'a') as u32);
                } else if destination.is_ascii_uppercase() {
                    if state & (1 << ((destination as u8 - b'A') as u32)) == 0 {
                        continue;
                    }
                }
                if d + d2 < *final_dist.get(&(destination, state2)).unwrap_or(&i32::MAX) {
                    final_dist.insert((destination, state2), d + d2);
                    pq.push(Reverse((d + d2, destination, state2)));
                }
            }
        }
        -1
    }
}
// @lc code=end
