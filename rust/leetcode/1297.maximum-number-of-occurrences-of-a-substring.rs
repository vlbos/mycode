/*
 * @lc app=leetcode id=1297 lang=rust
 *
 * [1297] Maximum Number of Occurrences of a Substring
 */

// @lc code=start
impl Solution {
    pub fn max_freq(s: String, max_letters: i32, min_size: i32, max_size: i32) -> i32 {
        let mut occ = std::collections::HashMap::new();
        let mut ans = 0;
        let sc = s.as_bytes();
        let min_size=min_size as usize;
        let max_letters = max_letters as usize;
        for i in 0..sc.len()-min_size+1{
             let cur = &sc[i..i+min_size];
             let exist = cur.iter().cloned().collect::<std::collections::HashSet<u8>>();
             if exist.len()<=max_letters{
                let curs:String  = String::from_utf8(cur.to_vec()).unwrap_or(String::new());
                *occ.entry(curs.clone()).or_insert(0)+=1;
                ans = ans.max(*occ.get(&curs).unwrap_or(&0));
              }
        }
        ans
    }
}
// @lc code=end

