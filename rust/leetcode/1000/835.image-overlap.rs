/*
 * @lc app=leetcode id=835 lang=rust
 *
 * [835] Image Overlap
 */

// @lc code=start
impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let n = img1.len();
        let mut count = vec![vec![0; 2 * n + 1]; 2 * n + 1];
        for i in 0..n {
            for j in 0..n {
                if img1[i][j] == 1 {
                    for r in 0..n {
                        for c in 0..n {
                            if img2[r][c] == 1 {
                                count[n + i - r][n + j - c] += 1;
                            }
                        }
                    }
                }
            }
        }
        *count
            .iter()
            .map(|x| x.iter().max().unwrap())
            .max()
            .unwrap()
    }
}
// @lc code=end
