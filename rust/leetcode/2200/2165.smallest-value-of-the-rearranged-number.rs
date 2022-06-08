/*
 * @lc app=leetcode id=2165 lang=rust
 *
 * [2165] Smallest Value of the Rearranged Number
 */

// @lc code=start
impl Solution {
    pub fn smallest_number(num: i64) -> i64 {
        if num >= -11 && num <= 11 {
            return num;
        }
        let mut ns: Vec<u8> = num.abs().to_string().bytes().collect();
        if num > 0 {
            ns.sort();
            let i = ns.iter().position(|x| *x > b'0').unwrap();
            ns.swap(0, i);
        } else {
            ns.sort_by(|a, b| b.cmp(&a));
        }
        let ans = String::from_utf8(ns).unwrap().parse::<i64>().unwrap();
        if num < 0 {
            ans * -1
        } else {
            ans
        }
    }
}
// @lc code=end
