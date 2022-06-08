/*
 * @lc app=leetcode id=686 lang=rust
 *
 * [686] Repeated String Match
 */

// @lc code=start
impl Solution {
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        if b.is_empty() {
            return 0;
        }
        if a == b {
            return 1;
        }
        let mut am = std::collections::HashSet::new();
        let mut bm = std::collections::HashSet::new();
        for c in a.chars() {
            am.insert(c);
        }
        for c in b.chars() {
            bm.insert(c);
        }
        if am.len() < bm.len() {
            return -1;
        }
        if !am.is_superset(&bm) {
            return -1;
        }
        let asize = if b.len() > a.len() {
            b.len() / a.len() + if (b.len() % a.len() > 0) { 1 } else { 0 }
        } else {
            1
        };
        let mut aa = a.repeat(asize);
        if let Some(i) = aa.find(&b) {
            asize as i32
        } else if let Some(i) = (aa+a.as_str()).find(&b){
                asize as i32+1
        }else{
            -1
        }
    }
}
// @lc code=end
