/*
 * @lc app=leetcode id=481 lang=rust
 *
 * [481] Magical String
 */

// @lc code=start
impl Solution {
    pub fn magical_string(n: i32) -> i32 {
        if n<4{
        return 1;
        }
        let mut i = 2;
        let mut j =2;
        let mut s = vec![b'1',b'2',b'2'];
        let mut ans = 1;
        while s.len()<n as usize{
               j= (s[i]-b'0') as usize;
               let  l = *s.last().unwrap();
               if l==b'1'{
                    s.extend(vec![b'2';j]);
                }else{
                        if s.len()+j>n as usize{
                            return ans+1;
                        }
                        ans+=j;
                        s.extend(vec![b'1';j]); 
                }
                i+=1;
        }
       
        ans as i32
    }
}
// @lc code=end

