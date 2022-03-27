/*
 * @lc app=leetcode id=1318 lang=rust
 *
 * [1318] Minimum Flips to Make a OR b Equal to c
 */

// @lc code=start
impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        let mut ans = 0;
        for i in 0..31{
            let (bit_a,bit_b,bit_c)=((a>>i)&1,(b>>i)&1,(c>>i)&1);
            if bit_c==0{
                ans+=bit_a+bit_b;
            }else if bit_a+bit_b==0{
                ans+=1;
            }
        }
        ans
    }
}
// @lc code=end

