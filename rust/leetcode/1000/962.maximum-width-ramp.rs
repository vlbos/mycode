/*
 * @lc app=leetcode id=962 lang=rust
 *
 * [962] Maximum Width Ramp
 */

// @lc code=start
impl Solution {
    pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
       let mut ni = nums.iter().enumerate().map(|(i,&v)|(i,v)).collect::<Vec<(usize,i32)>>();
       ni.sort_by_key(|x|x.1);
       let mut ans = 0;
       let mut m = ni.len() as i32;
       for  &(i,_) in &ni{
          ans = ans.max(i as i32-m);
          m = m.min(i as i32);
       }
        ans
    }
}
// @lc code=end

