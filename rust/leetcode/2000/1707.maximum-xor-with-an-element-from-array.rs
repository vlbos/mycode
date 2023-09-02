/*
 * @lc app=leetcode id=1707 lang=rust
 *
 * [1707] Maximum XOR With an Element From Array
 */

// @lc code=start
const L: usize = 30;
use std::collections::HashMap;
struct Trie {
    children: HashMap<i32, Option<Box<Trie>>>,
    min: i32,
}
impl Trie {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            min: i32::MAX,
        }
    }
    fn insert(&mut self, val: i32) {
       let mut node = self;
        node.min = node.min.min(val);
        for i in (0..L).rev() {
            let bit = (val >> i) & 1;
            node = node.children.entry(bit).or_insert(Some(Box::new(Trie::new()))).as_mut().unwrap();
            node.min = node.min.min(val);
        }
    }
    fn get_max_xor_with_limit(&self, val: i32, limit: i32) -> i32 {
        let mut node = self;
        if node.min > limit {
            return -1;
        }
        let mut ans = 0;
        let tmp=Some(Box::new(Trie::new()));
        for i in (0..L).rev() {
            let mut bit = (val >> i) & 1;
            if let Some(v) = node.children.get(&(bit ^ 1)) {
                if v.as_ref().unwrap().min <= limit {
                    ans |= 1 << i;
                    bit ^= 1;
                }
            }
            let nn =node.children.get(&bit).unwrap_or(&tmp);
            node =nn.as_ref().unwrap();
        }
        ans
    }
}
impl Solution {
    pub fn maximize_xor(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut t = Trie::new();
        for &v in &nums {
            t.insert(v);
        }
        let mut ans = Vec::new();
        for query in &queries {
            ans.push(t.get_max_xor_with_limit(query[0], query[1]));
        }
        ans
    }
}
// @lc code=end
impl Solution {
    pub fn maximize_xor(mut nums: Vec<i32>, mut queries: Vec<Vec<i32>>) -> Vec<i32> {
        nums.sort();
        queries=queries.into_iter().enumerate().map(|(i,mut v)|{ v.push(i as i32);v}).collect();
        queries.sort_by_key(|q|q[1]);
        let mut ans=vec![-1;queries.len()];
        #[derive(Default)]
        struct Trie{
            children:[Option<Box<Trie>>;2]
        }
        fn insert(mut node:&mut Trie,val:i32){
             for i in (0..30).rev(){
                 let bit=((val>>i)&1) as usize;
                 node=node.children[bit].get_or_insert(Box::new(Trie::default()));
             }
        }
         fn get_max_xor(mut node:&Trie,val:i32)->i32{
             let mut ans=0;
             for i in (0..30).rev(){
                 let mut bit=((val>>i)&1) as usize;
                 if node.children[1^bit].is_some(){
                     ans|=1<<i;
                     bit^=1;
                 }
                 if let Some(child)=&node.children[bit]{
                     node=child;
                 }
             }
             ans
        }
        let mut trie=Trie::default();
        let mut idx=0;
        let n=nums.len();
        for q in &queries{
            let (x,m,i)=(q[0],q[1],q[2]);
            while idx<n && nums[idx]<=m{
                insert(&mut trie,nums[idx]);
                idx+=1;
            }
           ans[i as usize]= if idx==0{
                -1
            }else{get_max_xor(&trie,x)};
        }
        ans
    }
}