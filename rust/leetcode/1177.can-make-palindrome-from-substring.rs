/*
 * @lc app=leetcode id=1177 lang=rust
 *
 * [1177] Can Make Palindrome from Substring
 */

// @lc code=start
impl Solution {
    pub fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
             let vec:Vec<i32>="a".bytes().chain(s.bytes()).scan(1_i32,|s,x|{*s^=1<<(x-b'a');Some(*s)}).collect();
        queries.into_iter().map(|x|(vec[x[1] as usize+1]^vec[x[0] as usize]).count_ones()>>1<=x[2] as u32).collect()
    }
}
// @lc code=end

