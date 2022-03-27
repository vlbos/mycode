/*
 * @lc app=leetcode.cn id=67 lang=rust
 *
 * [67] 二进制求和
 */

// @lc code=start
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut c = String::new();
        let mut inc: u8 = 0;
        let mut g = (&a, &b);
        if (a.chars().count() < b.chars().count()) {
            g = (&b, &a);
        }
        let mut bi = g.1.chars().rev();
        for ca in g.0.chars().rev() {
            let t = (ca) as u8 - 0x30
                + if let Some(cb) = bi.next() {
                    cb as u8 - 0x30
                } else {
                    0
                }
                + inc;
            // println!("{},{},{}",t ,ca as u16,(ca as u16).to_string());
            c.insert(0, (0x30 + (t % 2) as u8) as char);
            inc = t / 2;
            // println!("{},{},{}",inc,(t%2) ,(t%2));
        }
        if inc > 0 {
            c.insert(0, '1');
        }
        c
    }
}
// @lc code=end
