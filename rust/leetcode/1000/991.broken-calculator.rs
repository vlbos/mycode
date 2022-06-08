/*
 * @lc app=leetcode id=991 lang=rust
 *
 * [991] Broken Calculator
 */

// @lc code=start
impl Solution {
    pub fn broken_calc(start_value: i32, target: i32) -> i32 {
       let mut target = target;
        let mut ans = 0;
        while target>start_value{
                ans+=1;
                if target%2==0{
                target/=2;
                }else{
                    target+=1;
                }
        }
        ans+start_value-target
    }
}
// @lc code=end

