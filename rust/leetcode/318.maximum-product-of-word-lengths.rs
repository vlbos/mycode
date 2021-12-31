/*
 * @lc app=leetcode id=318 lang=rust
 *
 * [318] Maximum Product of Word Lengths
 */

// @lc code=start
impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
         let mut bm = std::collections::HashMap::<i32,usize>::new();
         let mut ans = 0;
         for w in words{
            let mut bit = 0;
            for b in w.bytes(){
                bit |= 1<< (b-b'a') as usize;
            }
            for (k,v) in &bm{
                if bit&k==0{
                    if ans < v*w.len(){
                        ans = v*w.len();
                    }
                }
            }
            if let Some(l) = bm.get_mut(&bit){
                    if *l<w.len(){
                        *l=w.len();
                    }
            }else{
                bm.insert(bit,w.len());
            }
         }
         ans as i32
    }
}
// @lc code=end

