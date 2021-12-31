/*
 * @lc app=leetcode id=1317 lang=rust
 *
 * [1317] Convert Integer to the Sum of Two No-Zero Integers
 */

// @lc code=start
impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        let mut m = 1;
        while n>m{
            if (m.to_string()+&(n-m).to_string()).find("0").is_none(){
                return vec![m,n-m].to_vec();
            }
            m+=1;
        }
        vec![m,n-m].to_vec()
    }
}
// @lc code=end

