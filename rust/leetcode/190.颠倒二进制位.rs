/*
 * @lc app=leetcode.cn id=190 lang=rust
 *
 * [190] 颠倒二进制位
 */

// @lc code=start
impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut r = 0;
        let mut x = x;
        let mut j = 0;
        while x>0{
            r += x%2;
            x/=2;
            if j==0 && x%2!=0{
                j=1;
            }
            else{
             j+=1;
            }
            if j<32{
            r<<=1;
         }

        }
        println!("{}",j);
        if (32-j-1)>0{
        r <<  (32-j-1)
        }
        else{r}
    }
}
// @lc code=end

