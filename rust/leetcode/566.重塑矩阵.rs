/*
 * @lc app=leetcode.cn id=566 lang=rust
 *
 * [566] 重塑矩阵
 */

// @lc code=start
impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
            if mat.is_empty()||mat.len()*mat[0].len()!= (r*c) as usize{
            return mat;
            }
            let mut r =  Vec::<Vec::<i32>>::new();
            let mut count = 0;
            let mut t = Vec::<i32>::new();
            for i in &mat{
            for j in i{
                count+=1;
                t.push(*j);
                if count%c==0{
                        r.push(t);
                        t = Vec::<i32>::new();
                }
            }
            }
            r
    }
}
// @lc code=end

