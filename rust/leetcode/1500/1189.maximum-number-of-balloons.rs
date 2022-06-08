/*
 * @lc app=leetcode id=1189 lang=rust
 *
 * [1189] Maximum Number of Balloons
 */

// @lc code=start
impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut a = vec![0; 26];
        let l = ('l' as u8 - 'a' as u8) as usize;
        let n = ('n' as u8 - 'a' as u8) as usize;
        let o = ('o' as u8 - 'a' as u8) as usize;
        for c in text.chars() {
            let mut i = (c as u8 - 'a' as u8) as usize;
            if i == 0 || i == 1 || i == l || i == n || i == o {
                a[i] += 1;
            }
        }
        a[0].min(a[1]).min(a[l] / 2).min(a[n]).min(a[o] / 2)
    }
}
// @lc code=end
