/*
 * @lc app=leetcode.cn id=118 lang=rust
 *
 * [118] 杨辉三角
 */

// @lc code=start
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut a = Vec::<Vec::<i32>>::new();
        for i in 0..num_rows{
            let mut b = Vec::<i32>::new();
            for j in 0..i+1{
                if 0==i{
                b.push(j+1);
                }
                else{
                    if 0==j || i==j{
                        b.push(1);
                    }else{
                        b.push(a[(i-1) as usize][(j-1)  as usize]+a[(i-1) as usize][j as usize]);
                    }
                }
            }
            a.push(b);
        }
        a
    }
}
// @lc code=end

