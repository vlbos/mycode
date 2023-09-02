/*
 * @lc app=leetcode id=1363 lang=rust
 *
 * [1363] Largest Multiple of Three
 */

// @lc code=start
impl Solution {
    pub fn largest_multiple_of_three(digits: Vec<i32>) -> String {
        let mut cnt = vec![0; 10];
        let mut modulo = vec![0; 3];
        let mut sum = 0;
        for &digit in &digits {
            cnt[digit as usize] += 1;
            modulo[digit as usize % 3] += 1;
            sum += digit;
        }
        let (mut remove_mod, mut rest) = if sum % 3 == 1 {
            if modulo[1] > 0 {
                (1, 1)
            } else {
                (2, 2)
            }
        } else if sum % 3 == 2 {
            if modulo[2] > 0 {
                (2, 1)
            } else {
                (1, 2)
            }
        } else {
            (0, 0)
        };
        let mut ans = String::new();
        for i in 0..=9 {
            for j in 0..cnt[i] {
                if rest > 0 && remove_mod == i % 3 {
                    rest -= 1;
                } else {
                    ans.push((b'0' + i as u8) as char);
                }
            }
        }
        if !ans.is_empty() && ans.chars().last().unwrap() == '0' {
            return "0".to_string();
        }
        ans.chars().rev().collect()
    }
}
// @lc code=end
impl Solution {
    pub fn largest_multiple_of_three(mut digits: Vec<i32>) -> String {
        digits.sort();
        let n=digits.len();
        let mut dp=vec![vec![0;3];n+1];
        dp[0][1]=i32::MIN/2;
        dp[0][2]=i32::MIN/2;
        for i in 1..=n{
            for j in (0..3).rev(){
                dp[i][j]=dp[i-1][j].max(dp[i-1][((j as i32-digits[i-1]%3+3)%3) as usize]+1);
            }
        }
        if dp[n][0]<=0{
           
            return String::new()
        }
        let mut ans=String::new();
        let mut digit=0;
        for i in (1..=n).rev(){
            if dp[i][digit]==dp[i-1][((digit as i32-digits[i-1]%3+3)%3) as usize]+1{
                ans.push((b'0'+digits[i-1] as u8) as char);
                digit=((digit as i32-digits[i-1]%3+3)%3) as usize;
                if ans.as_str()=="0"{
                    return ans
                }
            }
        }
        
        ans
    }
}