/*
 * @lc app=leetcode id=1419 lang=rust
 *
 * [1419] Minimum Number of Frogs Croaking
 */

// @lc code=start
impl Solution {
    pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
        let b = "croaks".as_bytes();
        let mut f = vec![0;b.len()];
        let mut ans = 0;
        let mut now = 0;
        for c in croak_of_frogs.bytes(){
            let i =b.iter().position(|x|*x==c).unwrap();
            f[i]+=1;
            if i==0{
                now+=1;
                ans=ans.max(now);

            }else if i==4{
            now-=1;
            }
            if f.windows(2).any(|x| x[0]<x[1]){
            return -1;
            }
        }
        if now==0{ans}else{-1}
    }
}
// @lc code=end

