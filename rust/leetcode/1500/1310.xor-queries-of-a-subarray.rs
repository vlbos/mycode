/*
 * @lc app=leetcode id=1310 lang=rust
 *
 * [1310] XOR Queries of a Subarray
 */

// @lc code=start
impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut xors=vec![0;arr.len()+1];
        for i in 0..arr.len(){
        xors[i+1]=xors[i]^arr[i];
        }
        let mut ans = vec![0;queries.len()];
        for (i,q) in queries.iter().enumerate(){
            ans[i]=xors[q[0] as usize]^xors[q[1] as usize+1];
        }
        ans
    }
}
// @lc code=end

