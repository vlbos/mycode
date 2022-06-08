/*
 * @lc app=leetcode id=85 lang=rust
 *
 * [85] Maximal Rectangle
 */

// @lc code=start
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut left = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == '1' {
                    left[i][j] = if j == 0 { 0 } else { left[i][j - 1] } + 1;
                }
            }
        }
        let mut ans = 0;
        for j in 0..n {
            let (mut up, mut down) = (vec![0; m], vec![0; m]);
            let mut s:Vec<usize> = Vec::new();
            for i in 0..m {
                while !s.is_empty() && left[s[s.len() - 1]][j] >= left[i][j] {
                    s.pop();
                }
                up[i] = if s.is_empty() {
                    -1
                } else {
                    s[s.len() - 1] as i32
                };
                s.push(i);
            }
            s = Vec::new();
            for i in (0..m).rev() {
                while !s.is_empty() && left[s[s.len() - 1]][j] >= left[i][j] {
                    s.pop();
                }
                down[i] = if s.is_empty() { m } else { s[s.len() - 1] } as i32;
                s.push(i);
            }
            for i in 0..m {
                ans = ans.max(( down[i] - up[i] -1) * left[i][j]);
            }
        }
        ans
    }
}
// @lc code=end
