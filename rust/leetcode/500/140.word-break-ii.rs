/*
 * @lc app=leetcode id=140 lang=rust
 *
 * [140] Word Break II
 */

// @lc code=start
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
       use std::collections::HashMap;
        use std::collections::HashSet;
        let mut ans = HashMap::new();
        let ws: HashSet<String> = word_dict.iter().cloned().collect();
        fn back_track(
            s: &String,
            index: usize,
            ws: &HashSet<String>,
            ans: &mut HashMap<usize, Vec<String>>,
        ) {
            if ans.contains_key(&index) {
                return;
            }
            if index == s.len() {
                ans.insert(index, vec![String::new()]);
                return;
            }
            ans.insert(index, Vec::new());
            for i in index + 1..=s.len() {
                let word = s[index..i].to_string();
                if ws.contains(&word) {
                    back_track(s, i, ws, ans);
                    let ss =  ans.get(&i).unwrap_or(&Vec::new()).clone() ;
                    for succ in ss{
                        ans.entry(index)
                            .or_insert(Vec::new())
                            .push(if succ.is_empty() {
                                word.clone()
                            } else {
                                word.clone() + " " + succ.as_str()
                            });
                    }
                }
            }
        }
        back_track(&s, 0, &ws, &mut ans);
        ans.get(&0).unwrap_or(&Vec::new()).clone()
    }
}
// @lc code=end
