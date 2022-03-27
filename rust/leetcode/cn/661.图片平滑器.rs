/*
 * @lc app=leetcode.cn id=661 lang=rust
 *
 * [661] 图片平滑器
 */

// @lc code=start
impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut averagegreys = img.clone();
        for (i, r) in img.iter().enumerate() {
            for (j, c) in r.iter().enumerate() {
                let mut count = 0;
                averagegreys[i][j] = 0;
                for x in if i == 0 { i } else { i - 1 }..=i + 1 {
                    for y in if j == 0 { j } else { j - 1 }..=j + 1 {
                        if x >= 0 && x <= img.len() - 1 && y >= 0 && y <= r.len() - 1 {
                            averagegreys[i][j] += img[x][y];
                            count += 1;
                        }
                    }
                }
                averagegreys[i][j] /= count;
            }
        }
        averagegreys
    }
}
// @lc code=end
