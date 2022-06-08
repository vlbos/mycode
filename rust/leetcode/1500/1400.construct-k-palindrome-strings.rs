/*
 * @lc app=leetcode id=1400 lang=rust
 *
 * [1400] Construct K Palindrome Strings
 */

// @lc code=start
impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        let r = s.len();
        let mut m= std::collections::HashMap::new();
        for c in s.chars(){
        *m.entry(c).or_insert(0)+=1;
        }
        let l = m.iter().filter(|x|x.1%2>0).count().max(1);
        let k = k as usize;
        l<=k && k<=r
    }
}
// @lc code=end

