/*
 * @lc app=leetcode id=1946 lang=rust
 *
 * [1946] Largest Number After Mutating Substring
 */

// @lc code=start
impl Solution {
    pub fn maximum_number(num: String, change: Vec<i32>) -> String {
         let n = num.len();
        let mut ans: Vec<u8> = num.bytes().collect();
        for i in 0..n {
            let j = (ans[i] - b'0') as usize;
            if change[j] > j as i32 {
                for k in i..n {
                    let j = (ans[k] - b'0') as usize;
                    if change[j] < j as i32 {
                        break;
                    }
                    ans[k] = b'0' + change[j] as u8;
                }
                break;
            }
        }
        String::from_utf8(ans).unwrap()
    }
}
// @lc code=end
