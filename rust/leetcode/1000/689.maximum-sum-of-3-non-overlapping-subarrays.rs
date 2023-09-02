/*
 * @lc app=leetcode id=689 lang=rust
 *
 * [689] Maximum Sum of 3 Non-Overlapping Subarrays
 */

// @lc code=start
impl Solution {
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut sum = vec![0; 3];
        let mut max_sum = vec![0; 3];
        let mut max_sum_idx = vec![0; 3];
        let k = k as usize;
        for i in 2 * k..nums.len() {
            for j in 0..3 {
                sum[j] += nums[i - k * (2 - j)];
            }
            if i >= k * 3 - 1 {
                if sum[0] > max_sum[0] {
                    max_sum[0] = sum[0];
                    max_sum_idx[0] = i - k * 3 + 1;
                }
                if max_sum[0] + sum[1] > max_sum[1] {
                    max_sum[1] = max_sum[0] + sum[1];
                    max_sum_idx[1] = max_sum_idx[0];
                    max_sum_idx[2] = i - k * 2 + 1;
                }
                if max_sum[1] + sum[2] > max_sum[2] {
                    max_sum[2] = max_sum[1] + sum[2];
                    ans = vec![
                        max_sum_idx[1] as i32,
                        max_sum_idx[2] as i32,
                        (i - k + 1) as i32,
                    ];
                }
                for j in 0..3 {
                    sum[j] -= nums[i - k * (3 - j) + 1];
                }
            }
        }
        ans
    }
}
// @lc code=end
impl Solution {
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let (n,k)=(nums.len(),k as usize);
        let mut sum:Vec<i32>=nums.iter().scan(0,|a,&x| {*a+=x;Some(*a)}).collect();
        sum.insert(0,0);
        let mut f=vec![vec![0;4];n+10];
        for i in (1..=n-k+1).rev(){
            for j in 1..4{
                f[i][j]=f[i+1][j].max(f[i+k][j-1]+sum[i+k-1]-sum[i-1]);
            }
        }
        let mut ans=vec![0;3];
        let (mut i,mut j,mut idx)=(1,3,0);
        while j>0{
            if f[i+1][j]>f[i+k][j-1]+sum[i+k-1]-sum[i-1]{
                i+=1;
            }else{
                ans[idx]=i as i32-1;
                idx+=1;
                i+=k;
                j-=1;
            }
        }
        ans
    }
}