/*
 * @lc app=leetcode.cn id=728 lang=rust
 *
 * [728] 自除数
 */

// @lc code=start
impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
      let mut v = Vec::<i32>::new();
      let   s = |nn:i32|->bool{
            if nn==0{
            return false;
            }
            let mut n = nn;
            if n%10==0{
                return false;
            }
            while n>0{
                 if n%10==0||nn%(n%10)>0{
                return false;
                 } 
                 
                 n/=10;
            }
            true
        };
      for n in left..=right{
            if s(n){
            v.push(n);
            }
      }
      v
    }
}
// @lc code=end

