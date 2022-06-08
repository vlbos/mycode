/*
 * @lc app=leetcode id=2014 lang=rust
 *
 * [2014] Longest Subsequence Repeated k Times
 */

// @lc code=start
impl Solution {
    pub fn longest_subsequence_repeated_k(s: String, k: i32) -> String {
        let check = |t: &String| {
            let bt = t.as_bytes();
            let mut cnt = 0;
            let mut j = 0;
            let (sn, tn) = (s.len(), t.len());
            for (i, b) in s.bytes().enumerate() {
                if tn * (k - cnt) as usize + i - j > sn {
                    return false;
                }
                if b != bt[j] {
                    continue;
                }
                j += 1;
                if j < tn {
                    continue;
                }
                cnt += 1;
                if cnt == k {
                    return true;
                }
                j = 0;
            }
            false
        };
        let mut vs = vec![Vec::new(); 8];
        vs[0].push(String::new());
        for i in 0..vs.len()-1 {
            for k in 0..vs[i].len() {
                for j in 0..26 {
                    let mut r = vs[i][k].clone();
                    r.push((b'a' + j) as char);
                    if check(&r) {
                        vs[i + 1].push(r);
                    }
                }
            }
        }
        for x in vs.iter().rev() {
            if !x.is_empty() {
                return x.iter().max().unwrap().clone();
            }
        }
        String::new()
    }
}
// @lc code=end
