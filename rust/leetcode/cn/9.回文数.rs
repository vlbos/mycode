/*
 * @lc app=leetcode.cn id=9 lang=rust
 *
 * [9] 回文数
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        // if x<0{
        // return false;
        // }
        // let mut xx :i64 = x as i64;
        // let mut result:i64 = 0;
        // while xx!=0{
        //     result = result*10+xx%10;
        //     xx /=10;
        // }
        // let max = i32::MAX as i64 ;
        // let min = i32::MIN as i64 ;
        // if result> max||result<min { false} else {x==(result as i32)}

        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }
        let mut xx = x;
        let mut result = 0;
        while xx > result {
            result = result * 10 + xx % 10;
            xx /= 10;
        }

        result == xx || xx == result / 10
    }
}
// @lc code=end
