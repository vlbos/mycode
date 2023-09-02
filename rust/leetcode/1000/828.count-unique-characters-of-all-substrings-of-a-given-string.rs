/*
 * @lc app=leetcode id=828 lang=rust
 *
 * [828] Count Unique Characters of All Substrings of a Given String
 */

// @lc code=start
impl Solution {
    pub fn unique_letter_string(s: String) -> i32 {
       let mut index = std::collections::HashMap::new();
        for (i, c) in s.chars().enumerate() {
            index.entry(c).or_insert(Vec::new()).push(i);
        }
        let mut ans = 0i64;
        for a in index.values() {
            for (i, &v) in a.iter().enumerate() {
                let v = v as i64;
                let prev = if i == 0 { -1 } else { a[i - 1] as i64};
                let next = if i == a.len() - 1 {
                    s.len() 
                } else {
                    a[i + 1] 
                } as i64;
                ans += (v - prev) * (next - v);
            }
        }
        ans as _
    }
}
// @lc code=end
impl Solution {
    pub fn unique_letter_string(s: String) -> i32 {
        let (n,mut ans,mut cur)=(s.len(),0,0);
        let (mut a,mut b)=(vec![-1;26],vec![-1;26]);
        let bs=s.as_bytes();
        for i in 0..n{
            let u=(bs[i]-b'A') as usize;
            cur+=i as i32-b[u]-(b[u]-a[u]);
            ans+=cur;
            a[u]=b[u];
            b[u]=i as i32;
        }
        ans
    }
}