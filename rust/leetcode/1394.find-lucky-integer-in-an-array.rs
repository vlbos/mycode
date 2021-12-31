/*
 * @lc app=leetcode id=1394 lang=rust
 *
 * [1394] Find Lucky Integer in an Array
 */

// @lc code=start
impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut r = vec![0;501];
        for a in &arr{
            r[*a as usize]+=1;
        }
        let mut m = 0;
        for (i,c) in r.iter().enumerate(){
            if *c as usize==i{
                if *c>m{
                    m=*c;
                }
            }
        }
        if m==0 {-1} else {m} 
    }
}
// @lc code=end

