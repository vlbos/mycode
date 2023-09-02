/*
 * @lc app=leetcode id=1178 lang=rust
 *
 * [1178] Number of Valid Words for Each Puzzle
 */

// @lc code=start
impl Solution {
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        let mut frequency = std::collections::HashMap::new();
        for word in &words {
            let mut mask:i32 = 0;
            for b in word.bytes() {
                mask |= 1 << (b - b'a');
            }
            if mask.count_ones() <= 7 {
                *frequency.entry(mask).or_insert(0) += 1;
            }
        }
        let mut ans = Vec::new();
        for puzzle in &puzzles {
            let mut total = 0;
            let mut mask = 0;
            for b in puzzle.bytes().skip(1) {
                mask |= 1 << (b - b'a');
            }
            let mut subset = mask;
            loop {
                let s = subset | (1 << (puzzle.as_bytes()[0] - b'a'));
                if let Some(&c) = frequency.get(&s) {
                    total += c;
                }
                subset = (subset - 1) & mask;
                if subset == mask {
                    break;
                }
            }
            ans.push(total);
        }
        ans
    }
}
// @lc code=end
impl Solution {
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        #[derive(Default)]
        struct Trie{
            children:[Option<Box<Trie>>;26],
            frequency:i32,
        }
        fn insert(mut node:&mut Trie,word:&[u8]){
            for &b in word{
                node=node.children[(b-b'a') as usize].get_or_insert(Box::new(Trie::default()));            
                }
                node.frequency+=1;
        }
        fn find(node:&Trie,pos:usize,required:u8,puzzle:&[u8])->i32{
            if pos==7{
                return node.frequency
            }
            let mut ans=if let Some(child)=&node.children[(puzzle[pos]-b'a') as usize]{
                find(child,pos+1,required,puzzle)
            }else{0};
            if puzzle[pos]!=required{
                ans+=find(node,pos+1,required,puzzle);
            }
            ans
        }
       let mut trie=Trie::default();
        let mut ans=Vec::new();
        for word in &words{
            let mut w:Vec<u8>=word.bytes().collect();
            w.sort();
            w.dedup();
            insert(&mut trie,&w);
        }
        for puzzle in &puzzles{
             let mut p:Vec<u8>=puzzle.bytes().collect();
              let required=p[0];
              p.sort();
            ans.push(find(&trie,0,required,&p));
        }
                   

        ans
    }
}
