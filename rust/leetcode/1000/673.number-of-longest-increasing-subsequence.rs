/*
 * @lc app=leetcode id=673 lang=rust
 *
 * [673] Number of Longest Increasing Subsequence
 */

// @lc code=start
impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut maxlen = 0;
        let mut dp = vec![0; nums.len()];
        let mut cnt = vec![0; nums.len()];
        for i in 0..nums.len() {
            dp[i] = 1;
            cnt[i] = 1;
            for j in 0..i {
                if nums[j] < nums[i] {
                    if dp[j] + 1 > dp[i] {
                        dp[i] = dp[j] + 1;
                        cnt[i] = cnt[j];
                    } else if dp[j] + 1 == dp[i] {
                        cnt[i] += cnt[j];
                    }
                }
            }
            if maxlen < dp[i] {
                maxlen = dp[i];
                ans = cnt[i];
            } else if maxlen == dp[i] {
                ans += cnt[i];
            }
        }
        ans
    }
}
// @lc code=end
impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let (mut d,mut cnt)=(Vec::<Vec<i32>>::new(),Vec::<Vec<i32>>::new());
        for &v in &nums{
            let i  = d.partition_point(|x| *x.last().unwrap()<v);
            let mut c=1;
            if i>0{
                let k=d[i-1].partition_point(|x| *x>=v);
                c=*cnt[i-1].last().unwrap()-cnt[i-1][k];
            }
            if i==d.len(){
                d.push(vec![v]);
                cnt.push(vec![0,c]);
            }else{
                d[i].push(v);
                let last=*cnt[i].last().unwrap()+c;
                cnt[i].push(last);
            }
        }
        *cnt.last().unwrap().last().unwrap()
    }
}