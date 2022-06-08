/*
 * @lc app=leetcode id=1542 lang=rust
 *
 * [1542] Find Longest Awesome Substring
 */

// @lc code=start
impl Solution {
    pub fn longest_awesome(s: String) -> i32 {
        let mut prefix = std::collections::HashMap::from([(0, -1)]);
        let mut ans = 0;
        let mut sequence = 0;
        for (i, b) in s.bytes().enumerate() {
            let digit = (b - b'0') as u32;
            sequence ^= (1 << digit);
            if let Some(&cnt) = prefix.get(&sequence) {
                ans = ans.max(i as i32 - cnt);
            } else {
                prefix.insert(sequence, i as i32);
            }
            for j in 0..10 {
                if let Some(&cnt) = prefix.get(&(sequence ^ (1 << j))) {
                    ans = ans.max(i as i32 - cnt);
                }
            }
        }
        ans
    }
}
// @lc code=end
