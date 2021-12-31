/*
 * @lc app=leetcode id=1331 lang=rust
 *
 * [1331] Rank Transform of an Array
 */

// @lc code=start
impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let mut a = Vec::new();
        let mut i = 0;
        let mut s = std::collections::HashSet::new();
        for n in &arr{
            if !s.contains(n){
            s.insert(*n);
            }
        }
        a = s.into_iter().collect::<Vec<i32>>();
        a.sort();
        let mut m = std::collections::HashMap::new();
        for (i,n) in a.iter().enumerate(){
            m.insert(*n,i as i32);
        }
        let mut ans = Vec::new();
        for n in &arr{
            ans.push(m.get(n).unwrap()+1);
        }
        ans
    }
}
// @lc code=end

