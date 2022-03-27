/*
 * @lc app=leetcode id=1405 lang=rust
 *
 * [1405] Longest Happy String
 */

// @lc code=start
impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut v: Vec<(usize, i32)> = [a, b, c]
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, v)| (i, v))
            .collect();
        let mut ans:Vec<usize> = Vec::new();
        loop {
            v.sort_by_key(|x| x.1);
            let (mid, most) = (v[v.len() - 2].0, v.last().unwrap().0);
            if v.last().unwrap().1>0 && (ans.len() < 2 || *ans.last().unwrap() != most || ans[ans.len() - 2] != most ){
                ans.push(most);
                v.last_mut().unwrap().1 -= 1;
            } else if v[v.len() - 2].1 > 0 {
                ans.push(mid);
                let n2 = v.len() - 2;
                v[n2].1 -= 1;
            } else {
                break;
            }
        }
        ans.iter().map(|x|(b'a'+((*x) as u8)) as char).collect()
    }
}
// @lc code=end
