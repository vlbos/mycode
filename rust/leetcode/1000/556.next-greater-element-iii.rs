/*
 * @lc app=leetcode id=556 lang=rust
 *
 * [556] Next Greater Element III
 */

// @lc code=start
impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        if n == i32::MAX {
            return -1;
        }
        let mut ns = n.to_string().chars().collect::<Vec<char>>();
        let mut j = ns.len();
        for i in (0..ns.len() - 1).rev() {
            if ns[i] < ns[i + 1] {
                j = i;
                break;
            }
        }
        if j == ns.len() {
            return -1;
        }
        let mut k = 0;
        for i in (0..ns.len()).rev() {
            if ns[i] > ns[j] {
                k = i;
                break;
            }
        }
        let t = ns[j];
        ns[j] = ns[k];
        ns[k] = t;
        let mut i = ns.len() - 1;
        j += 1;
        while j < i {
            let t = ns[j];
            ns[j] = ns[i];
            ns[i] = t;
            j += 1;
            i -= 1;
        }
       match ns.iter().collect::<String>().parse::<i32>(){Ok(v)=>v,_=>-1}
    }
}
// @lc code=end
