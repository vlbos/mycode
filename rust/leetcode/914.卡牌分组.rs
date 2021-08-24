/*
 * @lc app=leetcode.cn id=914 lang=rust
 *
 * [914] 卡牌分组
 */

// @lc code=start
impl Solution {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        if deck.len()<2{
        return false;
        }
        let mut dd = deck.clone();
        dd.sort();
        dd.dedup();
        let mut min = usize::MAX;
        for d in &dd{
            let c= deck.iter().filter(|v| **v==*d).count();
            min=min.min(c);
        }
        println!("{}",min);
fn gcd(x: usize, y: usize) -> usize {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}
        for d in &dd{
            let c= deck.iter().filter(|v| **v==*d).count();
            if gcd(c,min)<2{
            return false;
            }
            else{
                min=gcd(c,min);
            }
        }
        true
    }
}
// @lc code=end

