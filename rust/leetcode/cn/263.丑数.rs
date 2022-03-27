/*
 * @lc app=leetcode.cn id=263 lang=rust
 *
 * [263] 丑数
 */

// @lc code=start
impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        // n>0 &&  if n<=6 {true} else if n%2==0 {Self::is_ugly(n/2)}else if n%3==0 {Self::is_ugly(n/3)}else if n%5==0 {Self::is_ugly(n/5)} else {false}
        if n <= 0 {
            return false;
        }
        if n <= 6 {
            return true;
        }
        let ns = vec![2, 3, 5];
        for i in ns {
            if n % i == 0 {
                return Self::is_ugly(n / i);
            }
        }
        false
    }
}
// @lc code=end
