/*
 * @lc app=leetcode id=839 lang=rust
 *
 * [839] Similar String Groups
 */

// @lc code=start
impl Solution {
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        let n = strs.len();
        let mut f: Vec<usize> = (0..n).collect();
        fn find(x: usize, f: &mut Vec<usize>)->usize {
            if f[x] == x {
                return x;
            }
            f[x] = find(f[x],f);
            f[x]
        }
        let check = |a: &String, b: &String| -> bool {
            let mut num = 0;
            let bb = b.as_bytes();
            for (i, v) in a.bytes().enumerate() {
                if v != bb[i] {
                    num += 1;
                    if num > 2 {
                        return false;
                    }
                }
            }
            true
        };
        for i in 0..n {
            for j in i + 1..n {
                let (fi, fj) = (find(i, &mut f), find(j, &mut f));
                if fi == fj {
                    continue;
                }
                if check(&strs[i], &strs[j]) {
                    f[fi] = fj;
                }
            }
        }
        f.iter().enumerate().filter(|(i, &v)| *i == v).count() as _
    }
}
// @lc code=end
