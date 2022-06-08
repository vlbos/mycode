/*
 * @lc app=leetcode id=1903 lang=rust
 *
 * [1903] Largest Odd Number in String
 */

// @lc code=start
impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        let n = num.chars().collect::<Vec<char>>();
        if (n[n.len()-1] as u8 -'0' as u8)%2!=0{
            return num;
        }
        for i in (0..n.len()).rev(){
            if (n[i] as u8 -'0' as u8)%2!=0{
                return n[..i+1].iter().collect();
            }
        }
        "".to_string()
    }
}
// @lc code=end

