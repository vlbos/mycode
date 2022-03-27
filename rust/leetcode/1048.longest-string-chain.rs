/*
 * @lc app=leetcode id=1048 lang=rust
 *
 * [1048] Longest String Chain
 */

// @lc code=start
impl Solution {
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        let mut words =words;
        words.sort_by_key(|x|x.len());
        let mut m = std::collections::HashMap::new();
        let mut ans = 0;
        for w in &words{
            let mut best = 0;
            for i in 0..w.len(){
                let mut pre = w.clone();
                pre.remove(i);
                best = best.max(*(m.get(&pre).unwrap_or(&0))+1);
            }
            m.insert(w.clone(),best);
            ans = ans.max(best);
        }
        ans
    }
}
// @lc code=end

