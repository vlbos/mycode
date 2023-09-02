/*
 * @lc app=leetcode id=1238 lang=rust
 *
 * [1238] Circular Permutation in Binary Representation
 */

// @lc code=start
impl Solution {
    pub fn circular_permutation(n: i32, start: i32) -> Vec<i32> {
       let n2 = 1<<n;
        let mut ans = vec![0;n2];
        for i in 0..n2{
            let ii= i as i32;
            ans[i ]=ii^(ii>>1)^start;
        }
        ans
    }
}
// @lc code=end

impl Solution {
    pub fn circular_permutation(n: i32, start: i32) -> Vec<i32> {
        fn back_track(n: i32,upper_bound:usize,used:&mut Vec<bool>,ans:&mut Vec<i32>)->bool{
            let is_power_of_two=|n: i32|{n!=0 && (n-1)&n==0};
            if ans.len()==upper_bound && is_power_of_two(ans[0]^ans[ans.len()-1]){
                return true
            }
            for i in 0..n{
                let curr=(ans[ans.len()-1]^(1<<i as u32));
                if !used[curr  as usize] && is_power_of_two(curr^ans[ans.len()-1]){
                    used[curr  as usize]=true;
                    ans.push(curr);
                    if back_track(n,upper_bound,used,ans){
                        return true
                    }else{
                        ans.pop();
                        used[curr  as usize]=false;
                    }
                }
            }
            false
        }
        let upper_bound=1<<n as u32;
        let mut ans=vec![start];
        let mut used=vec![false;upper_bound];
        used[start as usize]=true;
        back_track(n,upper_bound,&mut used,&mut ans);
        ans
    }
}