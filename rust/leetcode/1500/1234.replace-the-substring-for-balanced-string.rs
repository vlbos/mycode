/*
 * @lc app=leetcode id=1234 lang=rust
 *
 * [1234] Replace the Substring for Balanced String
 */

// @lc code=start
impl Solution {
    pub fn balanced_string(s: String) -> i32 {
        let c2i="QWER".chars().enumerate().map(|(i,c)|(c,i)).collect::<std::collections::HashMap<char,usize>>();
        let mut count = [0; 4];
        let mut l = 0;
        let n = s.len();
        let mut ans = n;
        for c in s.chars() {
           count[c2i[&c]]+=1;
        }
        let s= s.chars().collect::<Vec<char>>();
        for r in 0..=n {
            while l <= r && 4 * count.iter().max().unwrap() - count.iter().sum::<usize>() <= r - l {
                ans = ans.min(r - l);
                count[c2i[&s[l.min(n - 1)]]]+=1;
                l += 1;
            }
             count[c2i[&s[r.min(n - 1)]]]-=1;
        }

        ans as _
    }
}
// @lc code=end

