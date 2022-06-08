/*
 * @lc app=leetcode id=423 lang=rust
 *
 * [423] Reconstruct Original Digits from English
 */

// @lc code=start
impl Solution {
    pub fn original_digits(s: String) -> String {
        let mut count=vec![0;26];
        for b in s.bytes(){
            count[(b-b'a') as usize]+=1;
        }
        let mut out=vec![0;10];
        let a=vec![b'z',b'w',b'u',b'x',b'g',b'h',b'f',b'v'];
        let c=vec![8,4,5];
        for i in 0..5{
            out[i*2]=count[(a[i]-b'a') as usize];
        }
        for i in 1..4{
            out[i*2+1]=count[(a[i+4]-b'a') as usize]-out[c[i-1]];
        }
        out[9]=count[(b'i'-b'a') as usize]-out[5]-out[6]-out[8];
        out[1]=count[(b'n'-b'a') as usize]-out[7]-2*out[9];
        let mut ans =String::new();
        for (i,v) in out.iter().enumerate(){
            ans.push_str(&(i.to_string()).repeat((*v) as usize));
        }
        ans
    }
}
// @lc code=end

