/*
 * @lc app=leetcode id=649 lang=rust
 *
 * [649] Dota2 Senate
 */

// @lc code=start
impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let n = senate.len();
        let mut r = std::collections::VecDeque::new();
        let mut d = std::collections::VecDeque::new();
        for (i, c) in senate.chars().enumerate() {
            if c == 'R' {
                r.push_back(i);
            } else {
                d.push_back(i);
            }
        }
        while !r.is_empty() && !d.is_empty() {
            match (r.front(), d.front()) {
                (Some(rv), Some(dv)) => {
                    if *rv < *dv {
                        r.push_back(*rv + n);
                    } else {
                        d.push_back(*dv + n);
                    }
                }
                _ => (),
            }

            r.pop_front();
            d.pop_front();
        }
        if !r.is_empty() {"Radiant".to_string()}else{"Dire".to_string()}
    }
}
// @lc code=end
