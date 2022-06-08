/*
 * @lc app=leetcode id=791 lang=rust
 *
 * [791] Custom Sort String
 */

// @lc code=start
impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut ans =String::new();
        let mut m = std::collections::HashMap::new();
        for c in s.chars(){
            *m.entry(c).or_insert(0)+=1;
        }
        for c in order.chars(){
            if let Some(n)=m.get(&c){
                ans.push_str(c.to_string().repeat(*n as usize).as_str());
                m.remove(&c);
            }
        }
        for (c,n) in &m{
                ans.push_str(c.to_string().repeat(*n as usize).as_str());
        }
        ans
    }
}
// @lc code=end

