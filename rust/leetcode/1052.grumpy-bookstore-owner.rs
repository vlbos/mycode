/*
 * @lc app=leetcode id=1052 lang=rust
 *
 * [1052] Grumpy Bookstore Owner
 */

// @lc code=start
impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let m = minutes as usize;
        let total = customers.iter().zip(&grumpy).filter(|x| *x.1==0).map(|x|x.0).sum::<i32>();
        let mut inc = customers.iter().take(m).zip(grumpy.iter().take(m)).map(|x|*x.0**x.1).sum::<i32>();
        let mut maxinc = inc;
        for i in m..customers.len(){
            let j = i-m;
            inc = inc- customers[j]*grumpy[j]+customers[i]*grumpy[i];
            maxinc = maxinc.max(inc);
        }
        total+maxinc
    }
}
// @lc code=end

