/*
 * @lc app=leetcode id=820 lang=rust
 *
 * [820] Short Encoding of Words
 */

// @lc code=start
impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut ws = words.iter().collect::<std::collections::HashSet<_>>();
        for w in &words{
            for i in 1..w.len(){
                  let mut ww = w.clone().split_off(i);
                   ws.remove(&ww);
            }
        }
        ws.iter().map(|x|x.len() as i32).sum::<i32>()+ws.len() as i32
    }
}
// @lc code=end

