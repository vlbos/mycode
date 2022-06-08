/*
 * @lc app=leetcode id=2007 lang=rust
 *
 * [2007] Find Original Array From Doubled Array
 */

// @lc code=start
impl Solution {
    pub fn find_original_array(changed: Vec<i32>) -> Vec<i32> {
       if changed.len() % 2 > 0 {
            return Vec::new();
        }
        let mut changed=changed;
        changed.sort();
        if changed.len() % 2 > 0 {
            return Vec::new();
        }
        let mut m=std::collections::HashMap::new();
        let mut ans=Vec::new();
        for &c in &changed {
                  if *m.get(&c).unwrap_or(&0)<=0{
               ans.push(c);
                 *m.entry(c*2).or_insert(0)+=1;
            }else{
                 *m.entry(c).or_insert(0)-=1;
            }
            if *m.get(&c).unwrap_or(&0)<=0{
                m.remove(&c);
            }
        }
        if !m.is_empty()  {
            return Vec::new();
        }

        ans
    }
}
// @lc code=end
