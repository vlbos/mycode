/*
 * @lc app=leetcode id=692 lang=rust
 *
 * [692] Top K Frequent Words
 */

// @lc code=start
impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut m = std::collections::HashMap::new();
        for w in &words{
             *m.entry(w.clone()).or_insert(0)+=1;
        }
        let mut ans=m.iter().map(|(x,y)|(x.clone(),*y)).collect::<Vec<(String,i32)>>();
        ans.sort_by(|x,y| if x.1>y.1 {std::cmp::Ordering::Less}else if x.1==y.1 { x.0.cmp(&y.0)}else{std::cmp::Ordering::Greater});
        ans.iter().map(|x|x.0.clone()).collect::<Vec<String>>()[..k as usize].to_vec()
    }
}
// @lc code=end

