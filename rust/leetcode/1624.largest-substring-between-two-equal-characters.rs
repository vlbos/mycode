/*
 * @lc app=leetcode id=1624 lang=rust
 *
 * [1624] Largest Substring Between Two Equal Characters
 */

// @lc code=start
impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut max = -1;
        let mut v = vec![-1;26];
        for (i,c) in s.chars().enumerate(){
            let j =  (c as u8 - 'a' as u8 ) as usize;
            if v[j]==-1{
                v[j]=i as i32;
            }else {
                max = max.max(i as i32 -v[j]-1);
            }
        }
        max
    }
}
// @lc code=end

