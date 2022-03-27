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

