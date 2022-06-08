/*
 * @lc app=leetcode id=1252 lang=rust
 *
 * [1252] Cells with Odd Values in a Matrix
 */

// @lc code=start
impl Solution {
    pub fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let m =m as usize;
        let mut r = vec![0;50];
        let mut c = vec![0;50];
        for i in &indices{
            r[i[0] as usize]+=1;
            c[i[1] as usize]+=1;
        }
        let ro = r.into_iter().filter(|x| *x%2!=0).count();
        let co = c.into_iter().filter(|x| *x%2!=0).count();
        (ro*(n-co)+co*(m-ro)) as i32
    }
}
// @lc code=end

