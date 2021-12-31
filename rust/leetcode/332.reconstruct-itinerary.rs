/*
 * @lc app=leetcode id=332 lang=rust
 *
 * [332] Reconstruct Itinerary
 */

// @lc code=start
impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut ans = Vec::new();
        let mut m = std::collections::HashMap::new();
        for t in tickets {
            let mut n = m.entry(t[0].clone()).or_insert(vec![]);
            n.push(t[1].clone());
            n.sort_by(|a, b| b.cmp(a));
        }
        let mut s = vec!["JFK".to_string()];
        while let Some(src) = s.last() {
            if let Some(mut v) = m.get_mut(src) {
                if let Some(n) = v.pop() {
                    s.push(n);
                    continue;
                }
            }
            if let Some(l) = s.pop() {
                ans.push(l);
            }
        }
        ans.reverse();
        ans
    }
}
// @lc code=end
