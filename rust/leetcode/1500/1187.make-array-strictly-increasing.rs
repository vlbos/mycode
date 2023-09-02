/*
 * @lc app=leetcode id=1187 lang=rust
 *
 * [1187] Make Array Strictly Increasing
 */

// @lc code=start
impl Solution {
    pub fn make_array_increasing(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let n = arr1.len();
        let mut dp = vec![vec![i32::MAX; n + 1]; n + 1];
        dp[0][0] = -1;
        let mut arr2 = arr2;
        arr2.sort();
        let k = arr2[..].partition_point(|&x| x<=3);

        for i in 1..=n {
            for j in 0..=i {
                if arr1[i - 1] > dp[j][i - 1] {
                    dp[j][i] = arr1[i - 1];
                }
                if j > 0 {
                    let k = arr2.partition_point(|x| *x <= dp[j - 1][i-1]);
                    if k >= 0 && k<arr2.len() {
                        dp[j][i] = dp[j][i].min(arr2[k]);
                    }
                }
                if i == n && dp[j][i] != i32::MAX {
                    return j as _;
                }
            }
        }
        -1
    }
}
// @lc code=end
impl Solution {
    pub fn make_array_increasing(mut arr1: Vec<i32>, mut arr2: Vec<i32>) -> i32 {
        arr2.sort();
        arr2.dedup();
        arr1.insert(0,-1);
        arr1.push(i32::MAX/2);
        let (n,m)=(arr1.len(),arr2.len());
        let mut dp=vec![i32::MAX/2;n];
        dp[0]=0;
        for i in 1..n{
            if arr1[i-1]<arr1[i]{
                dp[i]=dp[i].min(dp[i-1]);
            }
            let k=arr2.partition_point(|&x| x<arr1[i]);
            for j in 1..=k.min(i-1){
                if arr1[i-j-1]<arr2[k-j]{
                    dp[i]=dp[i].min(dp[i-j-1]+j as i32);
                }
            }
        }
        if dp[n-1]==i32::MAX/2{-1}else{dp[n-1]}
    }
}