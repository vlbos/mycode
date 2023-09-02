/*
 * @lc app=leetcode id=119 lang=rust
 *
 * [119] Pascal's Triangle II
 */

// @lc code=start
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
         let len = (row_index + 1) as usize;
        let mut a = Vec::<i32>::with_capacity(len);
        a.resize(len, 0);
        a[0] = 1;
        for i in 1..=row_index {
            let n = (row_index - i + 1) as i64;
            a[i as usize] = ((a[(i - 1) as usize] as i64) * n / (i as i64)) as i32;
        }
        a
    }
}
// @lc code=end

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let n=row_index as usize+1;
        let mut ans=vec![1;n];
        for i in 2..n{
            for j in (1..i).rev(){
                ans[j]+=ans[j-1];
            }
        }
        ans
    }
}