/*
 * @lc app=leetcode id=1442 lang=rust
 *
 * [1442] Count Triplets That Can Form Two Arrays of Equal XOR
 */

// @lc code=start
impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
       use std::collections::HashMap;
        let (mut cnt,mut total)=(HashMap::new(),HashMap::new());
        let mut ans = 0;
        let mut s=0;
        for (i,v) in arr.iter().enumerate(){
            let ii = i as i32;
            let sv = s^v;
            if let Some(j)=cnt.get(&sv){
                ans+=j*ii -*total.get(&sv).unwrap_or(&0);
            }
            *cnt.entry(s).or_insert(0)+=1;
            *total.entry(s).or_insert(0)+=ii;
            s^=v;
        }
        ans
    }
}
// @lc code=end

