/*
 * @lc app=leetcode id=1861 lang=rust
 *
 * [1861] Rotating the Box
 */

// @lc code=start
impl Solution {
    pub fn rotate_the_box(boxes: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let (m, n) = (boxes.len(), boxes[0].len());
        let mut boxes = boxes;
        let mut ans = vec![vec!['.'; m]; n];
        for i in 0..m {
            let mut p = n - 1;
            for j in (0..n).rev() {
                if boxes[i][j] == '*' {
                    if j == 0 {
                        break;
                    }
                    p = j - 1;
                } else if boxes[i][j] == '#' {
                    if p > j {
                        boxes[i][p] = '#';
                        boxes[i][j] = '.';
                        p -= 1;
                    } else {
                        p = j - 1;
                    }
                }
            }
        }
        for i in 0..m {
            for j in 0..n {
                ans[j][m - i - 1] = boxes[i][j];
            }
        }
        ans
    }
}
// @lc code=end
