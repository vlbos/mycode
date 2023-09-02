/*
 * @lc app=leetcode id=1286 lang=rust
 *
 * [1286] Iterator for Combination
 */

// @lc code=start
struct CombinationIterator {
chars:Vec<char>,
indies:Vec<usize>,
next:bool,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CombinationIterator {

    fn new(characters: String, combinationLength: i32) -> Self {
        Self{chars:characters.chars().collect(),indies : (0..combinationLength as usize).collect::<Vec<usize>>(),next:true}
    }
    
    fn next(&mut self) -> String {
        if !self.next{
            return String::new();
        }
        let mut ans = String::new();
        for i in 0..self.indies.len(){
                ans.push(self.chars[self.indies[i]]);
        }
        for i in (0..self.indies.len()).rev(){
             if self.indies[i]<self.chars.len()-(self.indies.len()-i){
                self.indies[i]+=1;
                for j in i+1..self.indies.len(){
                    self.indies[j]= self.indies[j-1]+1;
                }
                break;
             }else if i==0{
                self.next = false;
             }
        }
        ans
    }
    
    fn has_next(&mut self) -> bool {
        self.next
    }
}

/**
 * Your CombinationIterator object will be instantiated and called as such:
 * let obj = CombinationIterator::new(characters, combinationLength);
 * let ret_1: String = obj.next();
 * let ret_2: bool = obj.has_next();
 */
// @lc code=end

struct CombinationIterator {
vt:Vec<String>,
index:usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CombinationIterator {

    fn new(characters: String, combinationLength: i32) -> Self {
        fn back_track(i:usize,len:usize,cb:&[u8],tmp:&mut String,ans:&mut Vec<String>){
            if tmp.len()==len{
                ans.push(tmp.clone());
                return 
            }
            for j in i..cb.len(){
                tmp.push(cb[j] as char);
                back_track(j+1,len,cb,tmp,ans);
                tmp.pop();
            }
        }
        let mut vt=Vec::new();
        back_track(0,combinationLength as usize,characters.as_bytes(),&mut String::new(),&mut vt);
        Self{vt,index:0}
    }
    
    fn next(&mut self) -> String {
        self.index+=1;
        if self.index>self.vt.len(){
            return String::new()
        }
        self.vt[self.index-1].clone()
        
    }
    
    fn has_next(&self) -> bool {
        self.index<self.vt.len()
    }
}

/**
 * Your CombinationIterator object will be instantiated and called as such:
 * let obj = CombinationIterator::new(characters, combinationLength);
 * let ret_1: String = obj.next();
 * let ret_2: bool = obj.has_next();
 */