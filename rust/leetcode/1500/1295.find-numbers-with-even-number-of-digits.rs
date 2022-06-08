/*
 * @lc app=leetcode id=1295 lang=rust
 *
 * [1295] Find Numbers with Even Number of Digits
 */

// @lc code=start
impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut ans=0;
        for n in &nums{
            let mut nn = *n;
            let mut d = 0;
            while nn>0{
                d+=1;
                nn/=10;
            }
            if d>0 && d%2==0{
            ans+=1;
            }
        }
        ans
    }
}
// @lc code=end

