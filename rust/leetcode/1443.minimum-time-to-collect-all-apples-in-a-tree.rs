/*
 * @lc app=leetcode id=1443 lang=rust
 *
 * [1443] Minimum Time to Collect All Apples in a Tree
 */

// @lc code=start
impl Solution {
    pub fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
        let mut m=std::collections::HashMap::new();
        for e in edges.iter().rev(){
            *m.entry(e[0]).or_insert(0)=e[1];
            *m.entry(e[1]).or_insert(0)=e[0];

        }
        let mut ans = 0;
        let mut all_paths = std::collections::HashSet::new();
        let mut seen = std::collections::HashSet::new();
        for (i,&v) in has_apple.iter().enumerate(){
            if v{
               let mut j = i as i32;
                while j!=0 && !seen.contains(&j){
                    seen.insert(j);
                    all_paths.insert((m[&j],j));
                    j=m[&j];
                }
            }
        }
        all_paths.len() as i32 *2
    }
}
// @lc code=end

