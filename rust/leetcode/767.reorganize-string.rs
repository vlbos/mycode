/*
 * @lc app=leetcode id=767 lang=rust
 *
 * [767] Reorganize String
 */

// @lc code=start
impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let mut m =s.chars().fold(std::collections::HashMap::new(),|mut mm ,x|{*mm.entry(x).or_insert(0)+=1;mm});
        let max = *m.iter().max_by_key(|x|x.1).unwrap().1;
        if max>(s.len()+1)/2{
        return String::new();
        }
        let mut ans =vec![' ';s.len()];
        let mut oddindex =1;
        let mut evenindex=0;
        let halflen= s.len()/2;
        for (k,v) in m{
            let  mut cnt = v;
             while cnt > 0 && cnt <=halflen && oddindex<s.len(){
                ans[oddindex]=k;
                cnt-=1;
                oddindex+=2;
             }
             while cnt > 0 && evenindex<s.len(){
                ans[evenindex]=k;
                cnt-=1;
                evenindex+=2;
             }
        }

        ans.iter().collect()
    }
}
// @lc code=end

