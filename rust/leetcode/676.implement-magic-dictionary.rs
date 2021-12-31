/*
 * @lc app=leetcode id=676 lang=rust
 *
 * [676] Implement Magic Dictionary
 */

// @lc code=start
struct MagicDictionary {
words:std::collections::HashSet<String>,
count:std::collections::HashMap<String,i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MagicDictionary {

    fn new() -> Self {
        Self{words:std::collections::HashSet::new(),count:std::collections::HashMap::new()}
    }
    fn generalizedNeighbors(word:&String)->Vec<String>{
        let mut ans = Vec::new();
        let mut wc = word.chars().collect::<Vec<char>>();
        for i in 0..wc.len(){
            let letter = wc[i];
            wc[i]='*';
            ans.push(wc.iter().collect::<String>());
            wc[i]=letter;
        }
        ans
    }
    fn build_dict(&mut self, dictionary: Vec<String>) {
        for word in &dictionary{
                self.words.insert(word.clone());
                let ns = Self::generalizedNeighbors(word);
                for n in &ns{
                    *self.count.entry(n.clone()).or_insert(0)+=1;
                }
        }   
    }
    
    fn search(&self, search_word: String) -> bool {
        let ns = Self::generalizedNeighbors(&search_word);
        for n in &ns{
                let c = *self.count.get(n).unwrap_or(&0);
                if c>1 ||c==1&& !self.words.contains(&search_word){
                return true;
                }
        }
        false
    }
}

/**
 * Your MagicDictionary object will be instantiated and called as such:
 * let obj = MagicDictionary::new();
 * obj.build_dict(dictionary);
 * let ret_2: bool = obj.search(searchWord);
 */
// @lc code=end

