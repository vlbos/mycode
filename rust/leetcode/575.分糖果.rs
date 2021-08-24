/*
 * @lc app=leetcode.cn id=575 lang=rust
 *
 * [575] 分糖果
 */

// @lc code=start
impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
          let mut d = Vec::<i32>::new();
          for c in &candy_type{
              if !d.contains(c){
                    d.push(*c);
                }
          }
          d.len().min(candy_type.len()/2) as i32
    }
}
// @lc code=end

