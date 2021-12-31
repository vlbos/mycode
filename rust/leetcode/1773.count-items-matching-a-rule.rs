/*
 * @lc app=leetcode id=1773 lang=rust
 *
 * [1773] Count Items Matching a Rule
 */

// @lc code=start
impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let v = if rule_key =="type".to_string(){0} else if rule_key =="color".to_string(){1}else{2} as usize;
        items.iter().filter(|x| x[v] == rule_value).count() as i32
    }
}
// @lc code=end
