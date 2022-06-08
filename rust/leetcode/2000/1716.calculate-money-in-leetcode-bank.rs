/*
 * @lc app=leetcode id=1716 lang=rust
 *
 * [1716] Calculate Money in Leetcode Bank
 */

// @lc code=start
impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let mut nn = n;
        let mut i = 1;
        let mut sum = 0;
        let n7 = 28;
        while nn>=7{ 
            sum+= n7+(i-1)*7;
            nn-=7;
            i+=1;
        }
        nn=n%7;
        if nn>0{
             sum+= (1+nn)*nn/2+(i-1)*nn;
        }
        sum
    }
}
// @lc code=end

