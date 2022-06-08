/*
 * @lc app=leetcode id=739 lang=rust
 *
 * [739] Daily Temperatures
 */

// @lc code=start
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0;temperatures.len()];
        let mut s = Vec::new();
        for (i,t) in temperatures.iter().enumerate(){
              if s.is_empty(){
                s.push(i);
                continue;
              }
              while !s.is_empty() && *t>temperatures[*s.last().unwrap()]{
                    ans[*s.last().unwrap()]=(i-*s.last().unwrap()) as i32;
                    s.pop();
              }
              s.push(i);
        }
        ans
    }
}
// @lc code=end

