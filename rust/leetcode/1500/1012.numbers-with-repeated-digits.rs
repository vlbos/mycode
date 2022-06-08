/*
 * @lc app=leetcode id=1012 lang=rust
 *
 * [1012] Numbers With Repeated Digits
 */

// @lc code=start
impl Solution {
    pub fn num_dup_digits_at_most_n(n: i32) -> i32 {
       fn fact(n: i32) -> i32 {
            if n == 0 || n == 1 {
                1
            } else {
                n * fact(n - 1)
            }
        }
        let a = |m: i32, n: i32| fact(m) / fact(m - n);
        let ns = n.to_string();
        let nb = ns.as_bytes();
        let nn = nb.len();
        let mut used = vec![0; 10];
        let mut total = 0;
        for i in 1..nn {
            total += 9 * a(9, i as i32- 1);
        }
        for i in 0..nn {
            let num = (nb[i] - b'0') as usize;
            for j in if i == 0 { 1 } else { 0 }..num {
                if used[j] != 0 {
                    continue;
                }
                total += a((10 - i - 1) as i32, (nn - i - 1) as i32);
            }
            used[num] += 1;
            if used[num] > 1 {
                break;
            }
            if i == nn - 1 {
                total += 1;
            }
        }
        n - total
    }
}
// @lc code=end
