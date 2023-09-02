/*
 * @lc app=leetcode id=1849 lang=rust
 *
 * [1849] Splitting a String Into Descending Consecutive Values
 */

// @lc code=start
impl Solution {
    pub fn split_string(s: String) -> bool {
        let inf = 10i64.pow(10);
        let n = s.len();
        let bs = s.as_bytes();
        let mut start = 0;
        for i in 0..n - 1 {
            start = start * 10 + (bs[i] - b'0') as i64;
            if start >= inf {
                break;
            }
            let mut pval = start;
            let mut cval = 0;
            let mut cidx = i + 1;
            for j in i + 1..n {
                if cval >= inf {
                    break;
                }
                if pval == 1 {
                    if bs[cidx..].iter().all(|x| *x == b'0') {
                        return true;
                    } else {
                        break;
                    }
                }
                cval = cval * 10 + (bs[j] - b'0') as i64;
                if cval > pval - 1 {
                    break;
                }
                if cval == pval - 1 {
                    if j + 1 == n {
                        return true;
                    }
                    pval = cval;
                    cval = 0;
                    cidx = j + 1;
                }
            }
        }
        false
    }
}
// @lc code=end
impl Solution {
    pub fn split_string(s: String) -> bool {
        fn dfs(pos:usize,pre:i64,bs:&[u8])->bool{
            if pos==bs.len()-1{
                return true
            }
            if pre==0{
                return bs[pos+1..].iter().all(|&b| b==b'0')
            }
            let mut cur=0;
            for i in pos+1..bs.len(){
                cur=cur*10+(bs[i]-b'0') as i64;
                if cur==pre-1{
                    return dfs(i,cur,bs)
                }else if cur>=pre{
                    return false
                }
            }
            false
        }
        let bs=s.as_bytes();
        let mut i=0;
        while i<bs.len() && bs[i]==b'0'{
            i+=1;
        }
        let mut pre=0;
            while i<bs.len()-1{
                pre=pre*10+(bs[i]-b'0') as i64;
                if dfs(i,pre,bs){
                    return true
                }
                i+=1;
            }
            false
    }
}