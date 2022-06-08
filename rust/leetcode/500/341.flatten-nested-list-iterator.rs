/*
 * @lc app=leetcode id=341 lang=rust
 *
 * [341] Flatten Nested List Iterator
 */

// @lc code=start
// #[derive(Debug, PartialEq, Eq)]
// pub enum NestedInteger {
//   Int(i32),
//   List(Vec<NestedInteger>)
// }
struct NestedIterator {
    curr:usize,
    list:Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {

    fn new(nestedList: Vec<NestedInteger>) -> Self {
        let mut list =Vec::new();
        fn dfs(nestedList: &Vec<NestedInteger>,list:&mut Vec<i32>){
            for  n in nestedList{
                match n{
                NestedInteger::Int(x)=>list.push(*x),
                NestedInteger::List(l)=> dfs(&l,list),
                };
            }
        }
        dfs(&nestedList,&mut list);
        Self{curr:0,list}
    }
    
    fn next(&mut self) -> i32 {
        let i = self.curr;
        self.curr+=1;
        self.list[i]
    }
    
    fn has_next(&self) -> bool {
        !self.list.is_empty() && self.curr<self.list.len()
    }
}

/**
 * Your NestedIterator object will be instantiated and called as such:
 * let obj = NestedIterator::new(nestedList);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */
// @lc code=end

