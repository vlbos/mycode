/*
 * @lc app=leetcode id=417 lang=rust
 *
 * [417] Pacific Atlantic Water Flow
 */

// @lc code=start
impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = heights.len();
        let n = heights[0].len();
        let mut vispo = vec![vec![false; n]; m];
        let mut visao = vec![vec![false; n]; m];
        let mut poq = std::collections::VecDeque::new();
        let mut aoq = std::collections::VecDeque::new();
        for i in 0..m {
            poq.push_back(vec![i, 0]);
            aoq.push_back(vec![i, n - 1]);
            vispo[i][0] = true;
            visao[i][n - 1] = true;
        }
        for i in 0..n {
            poq.push_back(vec![0, i]);
            aoq.push_back(vec![m - 1, i]);
            vispo[0][i] = true;
            visao[m - 1][i] = true;
        }
        let mm = m as i32;
        let nn = n as i32;
        let directs = vec![vec![1, 0], vec![-1, 0], vec![0, -1], vec![0, 1]];
        while let Some(po) = poq.pop_front() {
            let i = po[0];
            let j = po[1];
            for d in &directs {
                let xx = i as i32 + d[0];
                let yy = j as i32 + d[1];
                let x = xx as usize;
                let y = yy as usize;
                if xx >= 0
                    && xx < mm
                    && yy >= 0
                    && yy < nn
                    && heights[x][y] >= heights[i][j]
                    && !vispo[x][y]
                {
                    poq.push_back(vec![x, y]);
                    vispo[x][y] = true;
                }
            }
        }
        while let Some(ao) = aoq.pop_front() {
            let i = ao[0];
            let j = ao[1];
            for d in &directs {
                let xx = i as i32 + d[0];
                let yy = j as i32 + d[1];
                let x = xx as usize;
                let y = yy as usize;
                if xx >= 0
                    && xx < mm
                    && yy >= 0
                    && yy < nn
                    && heights[x][y] >= heights[i][j]
                    && !visao[x][y]
                {
                    aoq.push_back(vec![x, y]);
                    visao[x][y] = true;
                }
            }
        }

        let mut ans = Vec::new();
        for i in 0..m {
            for j in 0..n {
                if visao[i][j] && vispo[i][j] {
                    ans.push(vec![i as i32, j as i32]);
                }
            }
        }
        ans
    }
}
// @lc code=end
