/*
 * @lc app=leetcode id=1263 lang=rust
 *
 * [1263] Minimum Moves to Move a Box to Their Target Location
 */

// @lc code=start
impl Solution {
    pub fn min_push_box(grid: Vec<Vec<char>>) -> i32 {
        use std::cmp::Reverse;
        let mut p = std::collections::BinaryHeap::new();
        let (m, n) = (grid.len() , grid[0].len() );
        let mut a = vec![0; 5];
        let mut grid = grid;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 'S' {
                    a[1] = i as i32;
                    a[2] = j as i32;
                    grid[i][j] = '.';
                } else if grid[i][j] == 'B' {
                    a[3] = i as i32;
                    a[4] = j as i32;
                    grid[i][j] = '.';
                }
            }
        }
        let mut visited = std::collections::HashSet::new();
        visited.insert(a[1..].to_vec());
        p.push(Reverse(a));
        let (m, n) = (m as i32, n as i32);
        let dirs = [0, 1, 0, -1, 0];
        while let Some(Reverse(v)) = p.pop() {
            for i in 0..dirs.len() - 1 {
                let (ns_x, ns_y) = (v[1] + dirs[i], v[2] + dirs[i + 1]);
                if ns_x < 0
                    || ns_x >= m
                    || ns_y < 0
                    || ns_y >= n
                    || grid[ns_x as usize][ns_y as usize] == '#'
                {
                    continue;
                }
                let (mut nb_x, mut nb_y) = (v[3], v[4]);
                let mut d = v[0];
                if ns_x == nb_x && ns_y == nb_y {
                    nb_x += dirs[i];
                    nb_y += dirs[i + 1];
                    if nb_x < 0
                        || nb_x >= m
                        || nb_y < 0
                        || nb_y >= n
                        || grid[nb_x as usize][nb_y as usize] == '#'
                    {
                        continue;
                    }
                    d += 1;
                }
                if grid[nb_x as usize][nb_y as usize] == 'T' {
                    return d;
                }
                if visited.contains(&(vec![ns_x, ns_y, nb_x, nb_y])) {
                    continue;
                }
                visited.insert(vec![ns_x, ns_y, nb_x, nb_y]);
                p.push(Reverse(vec![d, ns_x, ns_y, nb_x, nb_y]));
            }
        }
        -1
    }
}
// @lc code=end
