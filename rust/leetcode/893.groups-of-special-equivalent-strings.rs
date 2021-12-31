/*
 * @lc app=leetcode id=893 lang=rust
 *
 * [893] Groups of Special-Equivalent Strings
 */

// @lc code=start
impl Solution {
    pub fn num_special_equiv_groups(words: Vec<String>) -> i32 {
        let mut  seen = std::collections::HashSet::new();
        for w in &words{
              let mut count= vec![0;52];
              for (i,c) in w.bytes().enumerate(){
                    count[(c-b'a') as usize+26*(i%2) ]+=1;
              }
            seen.insert(count);
        }
        seen.len() as i32
    }
}
// @lc code=end

