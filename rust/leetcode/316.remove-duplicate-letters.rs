/*
 * @lc app=leetcode id=316 lang=rust
 *
 * [316] Remove Duplicate Letters
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
       
        let mut a = vec![0;26];
        for b in s.bytes(){
            a[(b-b'a') as usize]+=1;
         }
        let mut ans =  Vec::new();
        for b in s.bytes(){
            let ii = (b-b'a') as usize;
            if !ans.contains(&(b as char)){
                while  !ans.is_empty() && (*ans.last().unwrap()) as u8> b{
                    if a[((*ans.last().unwrap()) as u8-b'a') as usize]>0{
                        ans.pop();
                    }else{
                        break;
                    }
                }
                ans.push(b as char);
            }
            a[ii]-=1;
        }

        ans.iter().collect()
    }
}
// @lc code=end

