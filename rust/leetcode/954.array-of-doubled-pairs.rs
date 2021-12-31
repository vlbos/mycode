/*
 * @lc app=leetcode id=954 lang=rust
 *
 * [954] Array of Doubled Pairs
 */

// @lc code=start
impl Solution {
    pub fn can_reorder_doubled(arr: Vec<i32>) -> bool {
        let mut m  = std::collections::HashMap::new();
        for &a in &arr{
            *m.entry(a).or_insert(0)+=1;
        }
        let mut aa = arr.clone();
        aa.sort_by(|a,b|a.abs().cmp(&b.abs()));
        for a in &aa{
            if *m.get(a).unwrap_or(&0)==0{
                continue;
            }
            if *m.get(&(*a*2)).unwrap_or(&0)<=0{
               return false;
            }
            m.entry(*a).and_modify(|x|*x-=1);
            m.entry(*a*2).and_modify(|x|*x-=1);
        }
        
        true
    }
}
// @lc code=end

