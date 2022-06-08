/*
 * @lc app=leetcode id=2117 lang=rust
 *
 * [2117] Abbreviating the Product of a Range
 */

// @lc code=start
impl Solution {
    pub fn abbreviate_product(left: i32, right: i32) -> String {
        let mut ans = 1;
        let (mut two, mut five) = (0, 0);
        let mut f = false;
        for i in left..=right {
            ans = i as i64;
            while ans % 2 == 0 {
                ans /= 2;
                two += 1;
            }
            while ans % 5 == 0 {
                ans /= 5;
                five += 1;
            }
        }
        let d = two.min(five);
        two = d;
        five = d;
        let (mut low, mut low2) = (1, 1);
        for i in left..=right {
            ans = i as i64;
            while two > 0 && ans % 2 == 0 {
                ans /= 2;
                two -= 1;
            }
            while five > 0 && ans % 5 == 0 {
                ans /= 5;
                five -= 1;
            }
            low = low * ans % 10i64.pow(5);
            if !f {
                low2 *= ans;
                if low2 >= 10i64.pow(10) {
                    f = true;
                }
            }
        }
        if !f {
            return format!("{}e{}", low2, d);
        }
        let mut dd = 1.0;
        for i in left..=right {
            dd *= i as f64;
            while dd > 100_000.0 {
                dd /= 10.0;
            }
        }
        format!("{:.5}...{:05}e{}", dd.to_string(), low, d)
    }
}
// @lc code=end
