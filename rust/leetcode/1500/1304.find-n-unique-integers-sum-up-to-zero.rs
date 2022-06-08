/*
 * @lc app=leetcode id=1304 lang=rust
 *
 * [1304] Find N Unique Integers Sum up to Zero
 */

// @lc code=start
impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut r = vec![0;n].to_vec();
        for i in 0..n/2{
            let ii = (i+1) as i32;
            let i2=i*2;
            r[i2]=ii;
            r[i2+1]=-ii;
        }     
        if n%2!=0{
            r[n-1]=0;
        }    
        r            
    }
}
// @lc code=end

