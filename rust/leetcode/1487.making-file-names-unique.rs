/*
 * @lc app=leetcode id=1487 lang=rust
 *
 * [1487] Making File Names Unique
 */

// @lc code=start
impl Solution {
    pub fn get_folder_names(names: Vec<String>) -> Vec<String> {
        let mut m = std::collections::HashMap::<String,i32>::new();
        let mut ans = Vec::new();
        for n in &names {
            if let Some(i) = m.get(n) {
                let mut j=*i;
                while m.get(&format!("{}({})", n.clone(), j)).is_some(){
                    j+=1;
                }
                ans.push(format!("{}({})", n.clone(), j));
                m.insert(format!("{}({})", n.clone(), j),1);
                m.entry(n.clone()).and_modify(|x|*x=j);
            } else {
                ans.push(n.clone());
                m.insert(n.clone(),1);
            }
        }
        ans
    }
}
// @lc code=end
