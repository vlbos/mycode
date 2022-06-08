/*
 * @lc app=leetcode id=869 lang=rust
 *
 * [869] Reordered Power of 2
 */

// @lc code=start
use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let mut powerof2=HashSet::new();
        let mut i = 1;
        while i<=1000_000_000{
            let mut m = vec![0;10];
            for b in i.to_string().bytes(){
                m[(b-b'0') as usize]+=1;
            }
            powerof2.insert(m);
            i<<=1;
        }
        let mut m = vec![0;10];
        for b in n.to_string().bytes(){
                m[(b-b'0') as usize]+=1;
        }
        powerof2.contains(&m)
    }
}
// @lc code=end

