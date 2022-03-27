/*
 * @lc app=leetcode.cn id=204 lang=rust
 *
 * [204] 计数质数
 */

// @lc code=start
impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let mut is_primes = vec![true; n as usize].to_vec(); //Vec::<i32>::with_capacity(n as usize);
                                                             // let mut primes = Vec::<i32>::new();
                                                             // for i in 2..n{
                                                             //     if is_primes[i as usize]!=0{
                                                             //         primes.push(i);
                                                             //     }

        //     let mut j = 0;
        //     while j< primes.len() && i*primes[j]<n {
        //         is_primes[(i*primes[j]) as usize]=0;
        //         if i%primes[j]==0{
        //             break;
        //         }
        //         j+=1;
        //     }
        // }
        // primes.len() as i32

        let mut c = 0;
        let mut j = 0i64;
        let n = n as i64;
        let mut ii = 0i64;
        for i in 2..n {
            if is_primes[(i - 1) as usize] {
                c += 1;
                ii = i as i64;
                j = ii * ii;
                while j < n {
                    is_primes[(j - 1) as usize] = false;
                    j += ii;
                }
            }
        }
        c
    }
}
// @lc code=end
