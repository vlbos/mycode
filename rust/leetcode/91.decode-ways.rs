/*
 * @lc app=leetcode id=91 lang=rust
 *
 * [91] Decode Ways
 */

// @lc code=start
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let mut fi_2= 0;
        let mut fi_1= 1;
        let mut ans = 0;
        let sv = s.bytes().collect::<Vec<u8>>();
        let zero = '0' as u8;
        for i in 1..=sv.len(){
            ans = 0;
            if sv[i-1]!=zero{
                ans+=fi_1;
            }
            if i>1 && sv[i-2]!=zero && (sv[i-2]-zero)*10+(sv[i-1]-zero)<=26{
                ans+=fi_2;
            }
            fi_2=fi_1;
            fi_1=ans;
        }
        ans
    }
}
// @lc code=end

