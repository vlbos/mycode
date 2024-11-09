// [3339\. Find the Number of K-Even Arrays 🔒](https://leetcode.com/problems/find-the-number-of-k-even-arrays)
// ============================================================================================================



// [![](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)

// Description
// -----------

// You are given three integers `n`, `m`, and `k`.

// An array `arr` is called **k-even** if there are **exactly** `k` indices such that, for each of these indices `i` (`0 <= i < n - 1`):

// *   `(arr[i] * arr[i + 1]) - arr[i] - arr[i + 1]` is _even_.

// Return the number of possible **k-even** arrays of size `n` where all elements are in the range `[1, m]`.

// Since the answer may be very large, return it **modulo** `109 + 7`.

// **Example 1:**

// **Input:** n = 3, m = 4, k = 2

// **Output:** 8

// **Explanation:**

// The 8 possible 2-even arrays are:

// *   `[2, 2, 2]`
// *   `[2, 2, 4]`
// *   `[2, 4, 2]`
// *   `[2, 4, 4]`
// *   `[4, 2, 2]`
// *   `[4, 2, 4]`
// *   `[4, 4, 2]`
// *   `[4, 4, 4]`

// **Example 2:**

// **Input:** n = 5, m = 1, k = 0

// **Output:** 1

// **Explanation:**

// The only 0-even array is `[1, 1, 1, 1, 1]`.

// **Example 3:**

// **Input:** n = 7, m = 7, k = 5

// **Output:** 5832

// **Constraints:**

// *   `1 <= n <= 750`
// *   `0 <= k <= n - 1`
// *   `1 <= m <= 1000`


// int count_of_arrays(int n, int m, int k) 


#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn count_of_arrays(n: i32, m: i32, k: i32) -> i32 {
        let mut memo=vec![vec![vec![-1;2];k as usize+1];n as usize];
        fn dfs(i:usize,j:i32,k:i32,m:i64,memo:&mut Vec<Vec<Vec<i32>>>)->i32{
            if j<0{
            return 0
            }
            if i>=memo.len(){
                return if j==0{1}else{0}
            }
            if memo[i][j as usize][k as usize]!=-1{
return memo[i][j as usize][k as usize]}  
            let (cnt0,cnt1)=(m/2,(m+1)/2);       
            let a=cnt1*dfs(i+1,j,1,m,memo) as i64%1_000_000_007;
            let b=cnt0*dfs(i+1,j-(k&1^1),0,m,memo) as i64%1_000_000_007;
            memo[i][j as usize][k as usize]=((a+b)%1_000_000_007) as i32;
            memo[i][j as usize][k as usize]
        }
        dfs(0,k,1,m as i64,&mut memo)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_count_of_arrays_1() {
        assert_eq!(8, Solution::count_of_arrays(3, 4, 2));
    }
    #[test]
    pub fn test_count_of_arrays_2() {
        assert_eq!(1, Solution::count_of_arrays(5, 1, 0));
    }
    #[test]
    pub fn test_count_of_arrays_3() {
        assert_eq!(5832, Solution::count_of_arrays(7, 7, 5));
    }
}
