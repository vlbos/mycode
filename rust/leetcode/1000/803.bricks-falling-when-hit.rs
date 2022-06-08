/*
 * @lc app=leetcode id=803 lang=rust
 *
 * [803] Bricks Falling When Hit
 */

// @lc code=start
impl Solution {
    pub fn hit_bricks(grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
        let (h, w) = (grid.len(), grid[0].len());
        let n = h * w + 1;
        let mut f: Vec<usize> = (0..n).collect();
        let mut sz = vec![1; n];
        fn find(x: usize, f: &mut Vec<usize>) -> usize {
            if f[x] == x {
                return x;
            }
            f[x] = find(f[x], f);
            f[x]
        }
        fn merge(x: usize, y: usize, f: &mut Vec<usize>, sz: &mut Vec<i32>) {
            let (fx, fy) = (find(x, f), find(y, f));
            if fx == fy {
                return;
            }
            f[fx] = fy;
            sz[fy] += sz[fx];
        }
        let mut status = grid.clone();
        for hit in &hits {
            status[hit[0] as usize][hit[1] as usize] = 0;
        }
        for i in 0..h {
            for j in 0..w {
                if status[i][j] != 1 {
                    continue;
                }
                if i == 0 {
                    merge(h * w, i * w + j, &mut f, &mut sz);
                }
                if i > 0 && status[i - 1][j] == 1 {
                    merge(i * w + j, (i - 1) * w + j, &mut f, &mut sz);
                }
                if j > 0 && status[i][j - 1] == 1 {
                    merge(i * w + j, i * w + j - 1, &mut f, &mut sz);
                }
            }
        }
        let dirs = [0, 1, 0, -1, 0];
        let hn = hits.len();
        let mut ans = vec![0; hn];
        for i in (0..hn).rev() {
            let (r, c) = (hits[i][0] as usize, hits[i][1] as usize);
            if grid[r][c] == 0 {
                continue;
            }
            let prev = sz[find(h * w, &mut f)];
            if r == 0 {
                merge(c, h * w, &mut f, &mut sz);
            }
            for d in 0..dirs.len() - 1 {
                let (nr, nc) = (hits[i][0] + dirs[d], hits[i][1] + dirs[d + 1]);
                if nr < 0 || nr >= h as i32 || nc < 0 || nc >= w as i32 {
                    continue;
                }
                let (nr, nc) = (nr as usize, nc as usize);
                if status[nr][nc] == 1 {
                    merge(r * w + c, nr * w + nc, &mut f, &mut sz);
                }
            }
            let size = sz[find(h * w, &mut f)];
            ans[i] = 0i32.max(size - prev - 1);
            status[r][c] = 1;
        }
        ans
    }
}
// @lc code=end
