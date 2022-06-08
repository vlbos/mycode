/*
 * @lc app=leetcode id=659 lang=rust
 *
 * [659] Split Array into Consecutive Subsequences
 */

// @lc code=start
impl Solution {
    pub fn is_possible(nums: Vec<i32>) -> bool {
        let mut cm = std::collections::HashMap::new();
        let mut em = std::collections::HashMap::new();
        for n in &nums {
            *cm.entry(*n).or_insert(0) += 1;
        }

        for n in &nums {
            if let Some(cnt) = cm.get(n) {
                if *cnt > 0 {
                    if let Some(pre) = em.get_mut(&(*n - 1)) {
                        if *pre > 0 {
                            *cm.entry(*n).or_insert(0) -= 1;
                            *pre -= 1;
                            *em.entry(*n).or_insert(0) += 1;
                        } else {
                            match (cm.get(&(*n + 1)), cm.get(&(*n + 2))) {
                                (Some(cnt1), Some(cnt2)) if *cnt1 > 0 && *cnt2 > 0 => {
                                    *cm.entry(*n + 1).or_insert(0) -= 1;
                                    *cm.entry(*n + 2).or_insert(0) -= 1;
                                    *cm.entry(*n).or_insert(0) -= 1;
                                    *em.entry(*n + 2).or_insert(0) += 1;
                                }
                                _ => return false,
                            }
                        }
                    } else {
                        match (cm.get(&(*n + 1)), cm.get(&(*n + 2))) {
                            (Some(cnt1), Some(cnt2)) if *cnt1 > 0 && *cnt2 > 0 => {
                                *cm.entry(*n + 1).or_insert(0) -= 1;
                                *cm.entry(*n + 2).or_insert(0) -= 1;
                                *cm.entry(*n).or_insert(0) -= 1;
                                *em.entry(*n + 2).or_insert(0) += 1;
                            }
                            _ => return false,
                        }
                    }
                }
            }
        }
        true
    }
}
// @lc code=end
