/*
 * @lc app=leetcode id=3 lang=rust
 *
 * [3] Longest Substring Without Repeating Characters
 */

// @lc code=start
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max = 0;
        let mut h = std::collections::HashSet::new();
        let mut sv = s.chars().collect::<Vec<char>>();
        let mut j = 0;
        for i  in 0..sv.len(){
                if i>0{
                    h.remove(&sv[i-1]);
                }
                while j<sv.len() && !h.contains(&sv[j]){
                    h.insert(sv[j]);
                    j+=1;
                }
                max = max.max((j-i) as i32);
        }
        max 
    }
}
// @lc code=end
