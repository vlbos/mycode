/*
 * @lc app=leetcode id=754 lang=rust
 *
 * [754] Reach a Number
 */

// @lc code=start
impl Solution {
    pub fn reach_number(target: i32) -> i32 {
        let mut target = target.abs();
        let mut k = 0;
        while target>0{
            k+=1;
            target-=k;
        }
        if target%2==0{k}else{k+1+k%2}
    }
}
// @lc code=end

