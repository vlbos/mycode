/*
 * @lc app=leetcode.cn id=412 lang=rust
 *
 * [412] Fizz Buzz
 */

// @lc code=start
impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut r = Vec::<String>::new();
        for i in 1..=n {
            r.push(if i % 3 == 0 && i % 5 == 0 {
                "FizzBuzz".to_string()
            } else if i % 3 == 0 {
                "Fizz".to_string()
            } else if i % 5 == 0 {
                "Buzz".to_string()
            } else {
                i.to_string()
            })
        }
        r
    }
}
// @lc code=end
