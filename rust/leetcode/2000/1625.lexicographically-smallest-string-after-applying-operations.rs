/*
 * @lc app=leetcode id=1625 lang=rust
 *
 * [1625] Lexicographically Smallest String After Applying Operations
 */

// @lc code=start
impl Solution {
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        fn gcd(x: i32, y: i32) -> i32 {
            if y == 0 {
                return x;
            }
            gcd(y, x % y)
        }
        let n = s.len();
        let mut ans = s.clone();
        let g = gcd(n as i32, b) as usize;
        let ga = gcd(10, a) as usize;
        let ss = s.clone() + s.as_str();
        let bss = ss.as_bytes();
        let add = |q: &mut Vec<u8>, pos: usize| {
            let mut lo = (q[pos] - b'0') as usize;
            let mut added = 0;
            for l in (ga..10).step_by(ga) {
                let c = ((q[pos] - b'0') as usize + l) % 10;
                if c < lo {
                    lo = c;
                    added = l;
                }
            }
            if added > 0 {
                for l in (pos..n).step_by(2) {
                    q[l] = b'0' + (((q[l] - b'0') as usize + added) % 10) as u8;
                }
            }
        };
        for i in (0..n).step_by(g) {
            let p = &bss[i..i + n];
            let mut q = p.to_vec();
            add(&mut q, 1);
            if g % 2 > 0 {
                add(&mut q, 0);
            }
            ans = ans.min(String::from_utf8(q).unwrap());
        }
        ans
    }
}
// @lc code=end
