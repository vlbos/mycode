/*
 * @lc app=leetcode id=2080 lang=rust
 *
 * [2080] Range Frequency Queries
 */

// @lc code=start
use std::collections::HashMap;
struct RangeFreqQuery {
    occurence: HashMap<i32, Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RangeFreqQuery {
    fn new(arr: Vec<i32>) -> Self {
        let mut occurence = HashMap::new();
        for (i, &a) in arr.iter().enumerate() {
            occurence.entry(a).or_insert(Vec::new()).push(i as i32);
        }
        Self { occurence }
    }

    fn query(&self, left: i32, right: i32, value: i32) -> i32 {
        let d = Vec::new();
        let q = self.occurence.get(&value).unwrap_or(&d);
        let l = q.partition_point(|&x| x < left);
        let r = q.partition_point(|&x| x <= right);
        (r - l) as _
    }
}
/**
 * Your RangeFreqQuery object will be instantiated and called as such:
 * let obj = RangeFreqQuery::new(arr);
 * let ret_1: i32 = obj.query(left, right, value);
 */
// @lc code=end
use std::collections::HashMap;
struct RangeFreqQuery {
tree:Vec<(i32,i32,HashMap<i32,i32>)>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RangeFreqQuery {

    fn new(arr: Vec<i32>) -> Self {
        fn build(i:i32,l:i32,r:i32,arr: &Vec<i32>,tree:&mut Vec<(i32,i32,HashMap<i32,i32>)>){
            tree[i as usize].0=l;
            tree[i as usize].1=r;
            for j in l..=r{
                *tree[i as usize].2.entry(arr[j as usize]).or_insert(0)+=1;
            }
            if l==r{
                return
            }
            let mid=(l+r)/2;
            build(i*2,l,mid,arr,tree);
            build(i*2+1,mid+1,r,arr,tree);
        }
        let mut tree=vec![(0,0,HashMap::new());arr.len()*4];
        build(1,0,arr.len() as i32-1,&arr,&mut tree);
        Self{tree}
    }
    
    fn query(&self, left: i32, right: i32, value: i32) -> i32 {
         fn find(i:i32,l:i32,r:i32,x:i32,tree:&Vec<(i32,i32,HashMap<i32,i32>)>)->i32{
            if tree[i as usize].0>=l && 
            tree[i as usize].1<=r{
               return  *tree[i as usize].2.get(&x).unwrap_or(&0)
            }
            
            
            let mid=(tree[i as usize].0+ tree[i as usize].1)/2;
            let mut ans=0;
            if l<=mid{
                ans+=find(i*2,l,r,x,tree);
            }
           if mid<r{
                ans+=find(i*2+1,l,r,x,tree);
           }
            
            ans
        }
        find(1,left,right,value,&self.tree)
    }
}

/**
 * Your RangeFreqQuery object will be instantiated and called as such:
 * let obj = RangeFreqQuery::new(arr);
 * let ret_1: i32 = obj.query(left, right, value);
 */