/*
 * @lc app=leetcode id=1282 lang=rust
 *
 * [1282] Group the People Given the Group Size They Belong To
 */

// @lc code=start
impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut m = std::collections::HashMap::<i32,Vec<i32>>::new();
        let mut ans:Vec<Vec<i32>> = Vec::new();
        for (i,&v) in group_sizes.iter().enumerate(){
            if let Some(mut a)=m.get_mut(&v){
                if a.len() as i32==v {
                    ans.push(a.clone());
                    a.clear();
                }
            }
            m.entry(v).or_insert(Vec::new()).push(i as i32);
        }
        ans.extend(m.values().cloned().collect::<Vec<Vec<i32>>>());
        ans
    }
}
// @lc code=end

