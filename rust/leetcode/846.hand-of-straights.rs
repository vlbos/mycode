/*
 * @lc app=leetcode id=846 lang=rust
 *
 * [846] Hand of Straights
 */

// @lc code=start
impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        let mut m = std::collections::BTreeMap::new();
        for h in &hand {
            *m.entry(*h).or_insert(0) += 1;
        }
        while !m.is_empty() {
            if let Some(&first) = m.keys().nth(0) {
                for i in first..first + group_size {
                    if !m.contains_key(&i) {
                        return false;
                    }
                    m.entry(i).and_modify(|x| *x -= 1);
                    if *m.get(&i).unwrap_or(&-1) == 0 {
                        m.remove(&i);
                    }
                }
            }
        }
        true
    }
}
// @lc code=end
