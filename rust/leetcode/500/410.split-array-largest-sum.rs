/*
 * @lc app=leetcode id=410 lang=rust
 *
 * [410] Split Array Largest Sum
 */

// @lc code=start
impl Solution {
    pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
        let mut l = *nums.iter().max().unwrap();
        let mut r = nums.iter().sum::<i32>();
        let check = |x: i32| -> bool {
            let mut sum = 0;
            let mut cnt=1;
            for &num in &nums {
                if sum + num > x {
                    cnt += 1;
                    sum = num;
                } else {
                    sum += num;
                }
            }
            cnt <= m
        };
        while l < r {
            let mid = l + (r - l) / 2;
            if check(mid) {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        l
    }
}
// @lc code=end
impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let (n,k)=(nums.len(),k as usize);
        let mut f=vec![vec![i64::MAX;k+1];n+1];
        let mut sub=vec![0;n+1];
        for i in 0..n{
            sub[i+1]=sub[i]+nums[i] as i64;
        }
        f[0][0]=0;
        for i in 1..=n{
            for j in 1..=i.min(k){
                for m in 0..i{
                    f[i][j]=f[i][j].min(f[m][j-1].max(sub[i]-sub[m]))
                }
            }
        }
        f[n][k] as _
    }
}