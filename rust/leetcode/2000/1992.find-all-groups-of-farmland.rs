/*
 * @lc app=leetcode id=1992 lang=rust
 *
 * [1992] Find All Groups of Farmland
 */

// @lc code=start
impl Solution {
    pub fn find_farmland(land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (land.len(), land[0].len());
        let mut ans = Vec::new();
        let mut land = land;
        for i in 0..m {
            for j in 0..n {
                if land[i][j] > 0 {
                    let (mut r, mut c) = (i, j);
                    while r < m - 1 && land[r + 1][c] == 1 {
                        r += 1;
                    }
                    while c < n - 1 && land[r][c + 1] == 1 {
                        c += 1;
                    }
                    ans.push(vec![i as i32, j as i32, r as i32, c as i32]);
                    for x in i..=r {
                        for y in j..=c {
                            land[x][y] = 0;
                        }
                    }
                }
            }
        }
        ans
    }
}
// @lc code=end
