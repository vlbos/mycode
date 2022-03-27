/*
 * @lc app=leetcode.cn id=733 lang=rust
 *
 * [733] 图像渲染
 */

// @lc code=start
impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        let mut images = image.clone();
        let mut s = Vec::<(usize, usize)>::new();
        let mut ss = Vec::<(usize, usize)>::new();
        let xx = sr as usize;
        let yy = sc as usize;
        s.push((xx, yy));
        ss.push((xx, yy));
        while let Some(_s) = s.pop() {
            let x = _s.0 as usize;
            let y = _s.1 as usize;
            images[x][y] = new_color;
            if x > 0 {
                if image[xx][yy] == image[x - 1][y] {
                    if !ss.contains(&(x - 1, y)) {
                        s.push((x - 1, y));
                        ss.push((x - 1, y));
                    }
                }
            }
            if x < image.len() - 1 {
                if image[xx][yy] == image[x + 1][y] {
                    if !ss.contains(&(x + 1, y)) {
                        s.push((x + 1, y));
                        ss.push((x + 1, y));
                    }
                }
            }
            if y > 0 {
                if image[xx][yy] == image[x][y - 1] {
                    if !ss.contains(&(x, y - 1)) {
                        s.push((x, y - 1));
                        ss.push((x, y - 1));
                    }
                }
            }
            if y < image[0].len() - 1 {
                if image[xx][yy] == image[x][y + 1] {
                    if !ss.contains(&(x, y + 1)) {
                        s.push((x, y + 1));
                        ss.push((x, y + 1));
                    }
                }
            }
        }
        images
    }
}
// @lc code=end
