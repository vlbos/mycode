/*
 * @lc app=leetcode.cn id=883 lang=rust
 *
 * [883] 三维形体投影面积
 */

// @lc code=start
impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut mx = 0;
        let mut my = 0;
        let mut a = 0;
        for i in 0..grid.len(){
            mx=0;
            for j in 0..grid[i].len(){
                if grid[i][j]>mx{
                    mx=grid[i][j];
                }
            }
            a+=mx;
        }

        for j in 0..grid[0].len(){
            my=0;
            for i in 0..grid.len(){
                if grid[i][j]>my{
                    my=grid[i][j];
                }
                if grid[i][j]>0{
                    a+=1;
                }
            }
            a+=my;
        }
        a
    }
}
// @lc code=end

