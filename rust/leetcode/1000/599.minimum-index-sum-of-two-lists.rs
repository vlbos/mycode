/*
 * @lc app=leetcode id=599 lang=rust
 *
 * [599] Minimum Index Sum of Two Lists
 */

// @lc code=start
impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
         let mut r = Vec::<String>::new();
        let mut min = list1.len() + list2.len();
        for (i, l) in list1.iter().enumerate() {
            if let Some(j) = list2.iter().position(|v| *v == *l) {
                if i + j < min {
                    min = i + j;
                    r.clear();
                    r.push((*l).clone());
                } else if i + j == min {
                    r.push((*l).clone());
                }
            }
        }
        r
    }
}
// @lc code=end

