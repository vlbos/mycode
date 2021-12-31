/*
 * @lc app=leetcode id=904 lang=rust
 *
 * [904] Fruit Into Baskets
 */

// @lc code=start
impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut i = 0;
        let mut m = std::collections::HashMap::new();
        for j in 0..fruits.len(){
            *m.entry(fruits[j]).or_insert(0)+=1;
            while  m.len()>=3{
                m.entry(fruits[i]).and_modify(|x|*x-=1);
                if *m.get(&fruits[i]).unwrap_or(&0)==0{
                m.remove(&fruits[i]);
                }
                i+=1;
            }
            ans  = ans.max(j-i+1);
        }
        ans as _
    }
}
// @lc code=end

