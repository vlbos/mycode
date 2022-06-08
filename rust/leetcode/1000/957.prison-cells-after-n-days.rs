/*
 * @lc app=leetcode id=957 lang=rust
 *
 * [957] Prison Cells After N Days
 */

// @lc code=start
impl Solution {
    pub fn prison_after_n_days(cells: Vec<i32>, n: i32) -> Vec<i32> {
        let mut n = n;
        let next_day=|cells: &Vec<i32>|->Vec<i32>{
             let mut ans = vec![0;cells.len()];
             for i in 1..7{
                 if cells[i-1]==cells[i+1]{
                 ans[i]=1;
                }
             }
             ans
        };
        let mut m = std::collections::HashMap::new();
         let mut cells = cells;
        while n >0{
              if let Some(c)=m.get(&cells){
                   n%=c-n;
              }
              m.insert(cells.clone(),n);
              if n>0{
                n-=1;
                cells=next_day(&cells);
              }
        }
        cells
    }
}
// @lc code=end

