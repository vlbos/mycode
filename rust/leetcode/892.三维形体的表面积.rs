/*
 * @lc app=leetcode.cn id=892 lang=rust
 *
 * [892] 三维形体的表面积
 */

// @lc code=start
impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
            let mut r = 0;
            let mut ud=0;
            for i in 0..grid.len(){
                 for j in 0..grid[i].len(){
                        if grid[i][j]>0{
                                ud+=1;
                        }
                        if i==0{
                            r+=grid[i][j];
                        }else{
                            r+= if grid[i][j]-grid[i-1][j] > 0 {grid[i][j]-grid[i-1][j]}else{0};
                        }
                        if i==grid.len()-1{
                            r+=grid[i][j];
                        }else{
                            r+= if grid[i][j]-grid[i+1][j] > 0 {grid[i][j]-grid[i+1][j]}else{0};
                            }
                        if j==0{
                            r+=grid[i][j];
                        }else{
                            r+= if grid[i][j]-grid[i][j-1] > 0 {grid[i][j]-grid[i][j-1]}else{0};
                        }
                        if j==grid[i].len()-1{
                            r+=grid[i][j];
                        }else{
                            r+= if grid[i][j]-grid[i][j+1] > 0 {grid[i][j]-grid[i][j+1]}else{0};
                        }
                 }
            }
            r+ud*2
    }
}
// @lc code=end

