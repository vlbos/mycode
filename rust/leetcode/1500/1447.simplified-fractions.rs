/*
 * @lc app=leetcode id=1447 lang=rust
 *
 * [1447] Simplified Fractions
 */

// @lc code=start
impl Solution {
    pub fn simplified_fractions(n: i32) -> Vec<String> {
         let mut ans = std::collections::HashSet::new();
        fn gcd(a:i32,b:i32)->i32{
            if a==0{
                return b;
            }
            gcd(b%a,a)
        }
        for i in 1..n{
            for j in i+1..=n{
                let g = gcd(i,j);
                ans.insert(format!("{}/{}",i/g,j/g));
            }
        }
        ans.iter().cloned().collect()
    }
}
// @lc code=end

