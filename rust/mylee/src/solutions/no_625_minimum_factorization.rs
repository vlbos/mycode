// 625\. Minimum Factorization
// ===========================

// Given a positive integer `a`, find the smallest positive integer `b` whose multiplication of each digit equals to `a`.

// If there is no answer or the answer is not fit in 32-bit signed integer, then return 0.

// **Example 1**  
// Input:

// 48 

// Output:

// 68

// **Example 2**  
// Input:

// 15

// Output:

// 35

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Tencent](https://leetcode.ca/tags/#Tencent)

// @lc code=start
impl Solution {
    pub fn smallest_factorization(a: i32) -> i32 {
        // if a < 2 {
        //     return a;
        // }
        // let mut a = a as i64;
        // let mut res = 0i64;
        // let mut mul = 1i64;
        // for i in (2..=9).rev() {
        //     while a % i == 0 {
        //         a /= i;
        //         res = i * mul + res;
        //         mul *= 10;
        //     }
        // }
        // if a < 2 && res <= (i32::max_value() as i64) {
        //     res as i32
        // } else {
        //     0
        // }
        if 1==a{
        return a;
        }
        let mut a=a;
        let mut ans=String::new();
        for k in (2..=9).rev(){
            while a%k==0{
                ans=k.to_string()+ans.as_str();
            a/=k;
            }
        }
        if a>1{
        return 0;
        }
        let ans=ans.parse::<i64>().unwrap();
        if ans >i32::MAX as i64 {0}else {ans as _} 
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_smallest_factorization_1() {
        assert_eq!(Solution::smallest_factorization(48), 68);
    }

    #[test]
    fn test_smallest_factorization_2() {
        assert_eq!(Solution::smallest_factorization(15), 35);
    }

    #[test]
    fn test_smallest_factorization_3() {
        assert_eq!(Solution::smallest_factorization(i32::max_value()), 0);
    }
}
