/*
 * @lc app=leetcode id=833 lang=rust
 *
 * [833] Find And Replace in String
 */

// @lc code=start
impl Solution {
    pub fn find_replace_string(
        s: String,
        indices: Vec<i32>,
        sources: Vec<String>,
        targets: Vec<String>,
    ) -> String {
        let sv = s.chars().collect::<Vec<char>>();
        let mut ans = String::new();
        let mut start = 0;
        let n = indices.len();
        let mut orind = std::collections::HashMap::new();
        for (i, v) in indices.iter().enumerate() {
            orind.insert(*v, i);
        }
        let mut sortedindices = indices.clone();
        sortedindices.sort();
        for j in 0..sortedindices.len() {
            let i = *orind.get(&sortedindices[j]).unwrap_or(&0);
            let ii = indices[i] as usize;
            let len = sources[i].len();
            if sv[ii..ii + len].iter().collect::<String>() == sources[i] {
                ans.push_str(sv[start..ii].iter().collect::<String>().as_str());
                start = ii + len;
                ans.push_str(targets[i].as_str());
            }
        }
        if start < sv.len() {
            ans.push_str(sv[start..].iter().collect::<String>().as_str());
        }

        ans
    }
}
// @lc code=end
