/*
 * @lc app=leetcode id=675 lang=rust
 *
 * [675] Cut Off Trees for Golf Event
 */

// @lc code=start
impl Solution {
    pub fn cut_off_tree(forest: Vec<Vec<i32>>) -> i32 {
        let mut trees = forest
            .iter()
            .enumerate()
            .map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .filter(|x| *x.1 > 1)
                    .map(|(c, v)| (*v, r as i32, c as i32))
                    .collect::<Vec<(i32, i32, i32)>>()
            })
            .flatten().collect::<Vec<(i32, i32, i32)>>();
        trees.sort();
        let (mut sr, mut sc, mut ans) = (0, 0, 0);
        for &(_, tr, tc) in &trees {
            let d = bfs(&forest, sr, sc, tr, tc);
            if d < 0 {
                return -1;
            }
            ans += d;
            sr = tr;
            sc = tc;
        }
        fn bfs(forest: &Vec<Vec<i32>>, sr: i32, sc: i32, tr: i32, tc: i32) -> i32 {
            let (m, n) = (forest.len() as i32, forest[0].len() as i32);
            let mut q = std::collections::VecDeque::new();
            q.push_back((sr, sc, 0));
            let mut seen = std::collections::HashSet::from([(sr, sc)]);
            let dirs = [0, 1, 0, -1, 0];
            while let Some((r, c, d)) = q.pop_front() {
                if r == tr && c == tc {
                    return d;
                }
                for i in 0..dirs.len() - 1 {
                    let (nr, nc) = (r + dirs[i], c + dirs[i + 1]);
                    if nr < 0
                        || nr >= m
                        || nc < 0
                        || nc >= n
                        || seen.contains(&(nr, nc))
                        || forest[nr as usize][nc as usize] == 0
                    {
                        continue;
                    }
                    seen.insert((nr, nc));
                    q.push_back((nr, nc, d + 1));
                }
            }
            -1
        }
        ans
    }
}
// @lc code=end
