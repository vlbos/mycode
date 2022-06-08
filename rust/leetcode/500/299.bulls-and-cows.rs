/*
 * @lc app=leetcode id=299 lang=rust
 *
 * [299] Bulls and Cows
 */

// @lc code=start
impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let mut a = 0;
        let mut b = 0;
        let s = secret.bytes().collect::<Vec<u8>>();
        let g = guess.bytes().collect::<Vec<u8>>();
        let mut sp = vec![0;10];
        let mut gp = vec![0;10];
        for i in 0..s.len() {
            if s[i] == g[i] {
                a += 1;
            }else{
                sp[(s[i]-b'0') as usize]+=1;
                gp[(g[i]-b'0') as usize]+=1;
            }
        }
        for i in 0..10{
            b+=sp[i].min(gp[i]);
        }
        format!("{}A{}B", a, b)
    }
}
// @lc code=end
