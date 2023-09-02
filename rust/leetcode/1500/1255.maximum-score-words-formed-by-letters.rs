/*
 * @lc app=leetcode id=1255 lang=rust
 *
 * [1255] Maximum Score Words Formed by Letters
 */

// @lc code=start
impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut letter_cnt = HashMap::new();
        for &letter in &letters {
            *letter_cnt.entry(letter as u8 - b'a').or_insert(0) += 1;
        }
        let n = words.len();
        let group = |bit: i32| -> HashMap<u8, i32> {
            let mut g = HashMap::new();
            for i in 0..n {
                if bit & (1 << i) == 0 {
                    continue;
                }
                for b in words[i].bytes() {
                    *g.entry(b - b'a').or_insert(0) += 1;
                }
            }
            g
        };
        let calc_score = |g: &HashMap<u8, i32>| -> i32 {
            let mut ans = 0;
            for (j, &cnt) in g {
                if cnt > *letter_cnt.get(j).unwrap_or(&0) {
                    return 0;
                }
                ans += cnt * score[*j as usize];
            }
            ans
        };
        let mut ans = 0;
        for i in 0..(1 << n) {
            let g = group(i);
            ans = ans.max(calc_score(&g));
        }
        ans
    }
}
// @lc code=end
impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut left=HashMap::new();
        for &c in &letters{
            *left.entry(c).or_insert(0)+=1;
        }
        fn dfs(i:usize,mut total:i32,words: &Vec<String>, score: &Vec<i32>, left:&mut  HashMap<char,i32>,ans:&mut i32){
            if i==0{
                *ans=total.max(*ans);
                return
            }
            dfs(i-1,total,words,score,left,ans);
            for (j,c) in words[i-1].chars().enumerate(){
                if *left.get(&c).unwrap_or(&0)==0{
                    for c in words[i-1][..j].chars(){
                         *left.entry(c).or_insert(0)+=1;
                    }
                    return
                }
                *left.entry(c).or_insert(0)-=1;
                total+=score[(c as u8-b'a') as usize];
            }
            dfs(i-1,total,words,score,left,ans);
            for c in words[i-1].chars(){
                *left.entry(c).or_insert(0)+=1;
            }
        }
        let mut ans=0;
        dfs(words.len(),0, &words,&score,&mut left,&mut ans);
        ans
    }
}