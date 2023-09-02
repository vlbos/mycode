/*
 * @lc app=leetcode id=32 lang=rust
 *
 * [32] Longest Valid Parentheses
 */

// @lc code=start
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let (mut l, mut r) = (0, 0);
        let mut ans = 0;
        for c in s.chars() {
            if c == '(' {
                l += 1;
            } else {
                r += 1;
                if l == r {
                    ans = ans.max(2 * l);
                } else if r > l {
                    l = 0;
                    r = 0;
                }
            }
        }
        l = 0;
        r = 0;
        for c in s.chars().rev() {
            if c == ')' {
                r += 1;
            } else {
                l += 1;
                if l == r {
                    ans = ans.max(2 * l);
                } else if l > r {
                    l = 0;
                    r = 0;
                }
            }
        }
        ans
    }
}
// @lc code=end
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut ans=0;
        let mut dp=vec![0;s.len()];
        let bs=s.as_bytes();
        for i in 1..bs.len(){
            if bs[i]==b')'{
                if bs[i-1]==b'('{
                    dp[i]=2+if i<2{0}else{dp[i-2]};
                }else if i>dp[i-1] && bs[i-dp[i-1]-1]==b'('{
                    dp[i]=2+dp[i-1]+if i-dp[i-1]<2{0}else{dp[i-dp[i-1]-2]};
                }
                ans=ans.max(dp[i]);
            }
        }
        ans as _
    }
}