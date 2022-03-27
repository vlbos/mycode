/*
 * @lc app=leetcode id=1191 lang=rust
 *
 * [1191] K-Concatenation Maximum Sum
 */

// @lc code=start
impl Solution {
    pub fn k_concatenation_max_sum(arr: Vec<i32>, k: i32) -> i32 {
        let n = arr.len();
        let sum = arr.iter().sum::<i32>();
        let nn=k.min(2) as usize*n;
        let mut ans = 0;
        let mut cur = 0;
        for i in 0..nn{
            let val = arr[i%n] ;
            cur = (cur+val).max(0);
            ans=ans.max(cur)%1000_000_007;
        }
        if sum>0{
            for _ in 3..=k as usize{
                ans = (ans+sum)%1000_000_007;
            }
        }
        ans
    }
}
// @lc code=end

