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

struct MagicDictionary {
trie:Trie,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MagicDictionary {

    fn new() -> Self {
        Self{trie:Trie::new()}
    }
    
    fn build_dict(&mut self, dictionary: Vec<String>) {
        for word in &dictionary{
            self.trie.insert(word);
        }
    }
    
    fn search(&self, search_word: String) -> bool {
        self.trie.dfs(0,false,search_word.as_bytes())
    }
}

/**
 * Your MagicDictionary object will be instantiated and called as such:
 * let obj = MagicDictionary::new();
 * obj.build_dict(dictionary);
 * let ret_2: bool = obj.search(searchWord);
 */
 use std::collections::HashMap;
 struct Trie{
     children:HashMap<u8,Trie>,
     is_end:bool,
 }
 impl Trie{
     fn new()->Self{
         Self{children:HashMap::new(),is_end:false}
     }
     fn insert(&mut self,word:&String){
         let mut node=self;
         for c in word.bytes(){
             node=node.children.entry(c).or_insert(Trie::new());
         }
         node.is_end=true;
     }
     fn dfs(&self,pos:usize,modified:bool,word:&[u8])->bool{
         let mut node=self;
         if pos==word.len(){
             return modified && node.is_end
         }
        let ch=word[pos];
        if let Some(child)=node.children.get(&ch){
            if child.dfs(pos+1,modified,word){
                return true
            }
        }
        if !modified{
              for (&cnext,child) in &node.children{
                  if ch!=cnext{
                        if child.dfs(pos+1,true,word){
                            return true
                        }
                  }
                 
             }
        }
        false
     }
 }
