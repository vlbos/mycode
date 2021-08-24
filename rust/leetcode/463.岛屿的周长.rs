/*
 * @lc app=leetcode.cn id=463 lang=rust
 *
 * [463] 岛屿的周长
 */

// @lc code=start
impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut s = 0;
        for (i,r) in grid.iter().enumerate(){
            for (j,c) in r.iter().enumerate(){
                    if *c==1{
                        if i==0{
                            s+=1;
                        }
                        else if grid[i-1][j]==0{
                            s+=1;
                        }
                        if i==grid.len()-1{
                            s+=1;
                        }else if grid[i+1][j]==0{
                            s+=1;
                        }

                        if j==0{
                            s+=1;
                        } else if grid[i][j-1]==0{
                            s+=1;
                        }
                        if j==r.len()-1{
                            s+=1;
                        } else if grid[i][j+1]==0{
                            s+=1;
                        }
                    }
                        
            }
        }
        s
    }
}
// @lc code=end

