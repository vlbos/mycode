/*
 * @lc app=leetcode id=1704 lang=rust
 *
 * [1704] Determine if String Halves Are Alike
 */

// @lc code=start
impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let v= vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        let ssc = |x:&str|->i32{  
            let mut cnt = 0;
            for c in x.chars(){
                if v.contains(&c){
                    cnt +=1;
                }
            }
            cnt
        };
        ssc(&s[..s.len()/2])==ssc(&s[s.len()/2..])
    }
}
// @lc code=end

