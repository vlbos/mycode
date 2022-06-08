/*
 * @lc app=leetcode id=648 lang=rust
 *
 * [648] Replace Words
 */

// @lc code=start
impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let d  = dictionary.iter().collect::<std::collections::HashSet<_>>();
        let mut ans = Vec::new();
        for ss in sentence.split_ascii_whitespace(){
              let sv = ss.chars().collect::<Vec<char>>();
              let mut flag = true;
              for i in 1..sv.len(){
                    let subs = &sv[0..i].iter().collect::<String>();
                    if d.contains(&subs){
                        ans.push(subs.clone());
                        flag=false;
                        break;
                    }
              }
              if flag{
                        ans.push(ss.to_string().clone());
              }
        }
        ans.join(" ")
    }
}
// @lc code=end

