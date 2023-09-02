/*
 * @lc app=leetcode id=1653 lang=rust
 *
 * [1653] Minimum Deletions to Make String Balanced
 */

// @lc code=start
impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let mut a = vec![0; 2];
        for b in s.bytes() {
            a[(b - b'a') as usize] += 1;
        }
        if a[0]==0||a[1]==0{
            return 0;
        }
        let mut aa = vec![0; 2];
        let mut ans = i32::MAX;
        ans = ans.min(aa[1] +  a[0]);
        for (i, b) in s.bytes().enumerate() {
            let j = (b - b'a') as usize;
            aa[j] += 1;
            a[j] -= 1;
            ans = ans.min(aa[1] +  a[0]);
        }
        ans
    }
}
// @lc code=end
impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let (mut ans,mut b)=(0,0);
        for c in s.chars(){
            if c=='b'{
                b+=1;
            }else{
                ans=b.min(ans+1);
            }
        }
        ans
    }
}