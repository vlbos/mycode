/*
 * @lc app=leetcode id=1006 lang=rust
 *
 * [1006] Clumsy Factorial
 */

// @lc code=start
impl Solution {
    pub fn clumsy(n: i32) -> i32 {
       let mut s = Vec::new();
        let mut n = n;
        s.push(n);
        n-=1;
        let mut i = 0;
        while n>0{
            let v = match i%4{
                0=>s.pop().unwrap()*n,
                1=>s.pop().unwrap()/n,
                2=>n,
                _=>-n,
            };
            s.push(v);
            n-=1;
            i+=1;
        }
        s.iter().sum::<i32>()
    }
}
// @lc code=end

