/*
 * @lc app=leetcode id=1175 lang=rust
 *
 * [1175] Prime Arrangements
 */

// @lc code=start
impl Solution {
    pub fn num_prime_arrangements(n: i32) -> i32 {
        let n = n as i64;
        let m = 1e9 as i64 +7;
        let isPrime=|x:i64|->bool{
            if x<=1{
            return false;
            }
            let mut i = 2;
            while i*i<=x{
                if x%i==0{
                return false;
                }
                i+=1;
            }
            true
        };
        let factorial = |x:i64|->i64{
            let mut r = 1;
            for i in 2..=x{
                r*=i;
                r%=m;
            }
            r
        };
        let mut cnt = 0;
        for i in 2..=n{
                if isPrime(i){
                    cnt+=1;
                }
        }
        ((factorial(cnt)*factorial(n-cnt))%m) as i32
    }
}
// @lc code=end

