/*
 * @lc app=leetcode.cn id=832 lang=rust
 *
 * [832] 翻转图像
 */

// @lc code=start
impl Solution {
    pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
let  rl = image.len();
let  cl = image[0].len();
        let mut im = image;
        for j in 0..rl{
        for i in 0..cl/2{
            let t = im[j][i];
            im[j][i]=im[j][cl-i-1];
            im[j][cl-i-1]= t;
        }
         for i in 0..cl{
            im[j][i]+=1;
            im[j][i]%=2;
        }
        }
        im
    }
}
// @lc code=end

