/*
 * @lc app=leetcode id=748 lang=rust
 *
 * [748] Shortest Completing Word
 */

// @lc code=start
impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let mut t = vec![0; 26];
        let mut cc = String::new();
        for c in license_plate.chars() {
            if c.is_ascii_alphabetic() {
                let cl = c.to_ascii_lowercase();
                cc = cc + &cl.to_string();
                t[(cl as u8 - 'a' as u8) as usize] += 1;
            }
        }
        let count = |s: String| -> Vec<i32> {
            let mut t = vec![0; 26];
            for c in s.chars() {
                if c.is_ascii_alphabetic() {
                    let cl = c.to_ascii_lowercase();
                    t[(cl as u8 - 'a' as u8) as usize] += 1;
                }
            }
            t.to_vec()
        };
        let don = |w: String| -> bool {
            let c = count(w);
            for i in 0..26 {
                if t[i] > c[i] {
                    return false;
                }
            }
            true
        };

        let mut ret = String::new();
        for w in &words {
            let len = (*w).len();
            if (ret.is_empty() || len < ret.len()) && len >= cc.len() && don((*w).clone()) {
                ret = (*w).clone();
            }
        }
        ret
    }
}
// @lc code=end

