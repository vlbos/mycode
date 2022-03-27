/*
 * @lc app=leetcode id=1663 lang=rust
 *
 * [1663] Smallest String With A Given Numeric Value
 */

// @lc code=start
impl Solution {
    pub fn get_smallest_string(n: i32, k: i32) -> String {
        let b = k / n;
        let mut a = vec![0; n as usize];
        let mut c = k-n;
        for i in (0..n as usize).rev() {
            if c >= 25 {
                a[i] += 25;
                c -= 25;
                if c == 0 {
                break;
                }
            }else{
                a[i] += c;
                break;
            }
        }
        a.iter()
            .map(|&x| ((x as u8) + b'a')  as char)
            .collect::<String>()
    }
}
// @lc code=end
