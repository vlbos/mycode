/*
 * @lc app=leetcode id=307 lang=rust
 *
 * [307] Range Sum Query - Mutable
 */

// @lc code=start
struct NumArray {
tree:Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let mut tree = vec![0;n*2];
        for i in 0..n{
             tree[n+i]=nums[i];
        }
        for i in (1..n).rev(){
             tree[i]=tree[2*i]+tree[2*i+1];
        }

        Self{tree}
    }
    
    fn update(&mut self, index: i32, val: i32) {
        let mut index = index as usize+self.tree.len()/2;
        self.tree[index]=val;
        while index >0{
            let mut l = index;
            let mut r = index;
            if index%2==0{
                r+=1;
            }else{
                l-=1;
            }
            self.tree[index/2]=self.tree[l]+self.tree[r];
            index/=2;
        }
    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let mut sum = 0;
        let mut l = left as usize+self.tree.len()/2;
        let mut r = right as usize+self.tree.len()/2;
        while l<=r{
            if l%2!=0{
                sum+=self.tree[l];
                l+=1;
            }
            if r%2==0{
                sum+=self.tree[r];
                r-=1;
            }
            l/=2;
            r/=2;
        }
        sum
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(index, val);
 * let ret_2: i32 = obj.sum_range(left, right);
 */
// @lc code=end

struct NumArray {
nums: Vec<i32>,
tree: Vec<i32>,

}

////Solution 2  Binary Indexed Tree
/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        let mut tree=vec![0;nums.len()+1];
        for (i,&num) in nums.iter().enumerate(){
            Self::add(&mut tree,i as i32+1,num);
        }
        Self{nums,tree}
    }
    fn add(tree:&mut Vec<i32>,mut index: i32, val: i32) {
        let n=tree.len() as i32;
        while index<n{
            tree[index as usize]+=val;
            index+=-index&index;
        }
    }
    
    fn update(&mut self, index: i32, val: i32) {
        Self::add(&mut self.tree,index+1,val-self.nums[index as usize]);
        self.nums[index as usize]=val;
    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let prefix_sum=|mut index: i32|{
            let mut s=0;
            while index>0{
                s+=self.tree[index as usize];
                index&=index-1;
            }
            s
        };
        prefix_sum(right+1)-prefix_sum(left)
    }
}


////Solution 3  Segment Tree

struct NumArray {
seg: Vec<i32>,

}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        let n=nums.len();
        let mut seg=vec![0;n*4];
        fn build(nums: &Vec<i32>,seg:&mut Vec<i32>,node:usize,s:usize,e:usize){
            if s==e{
                seg[node]=nums[s];
                return 
            }
            let m=s+(e-s)/2;
            build(nums,seg,node*2+1,s,m);
            build(nums,seg,node*2+2,m+1,e);
            seg[node]=seg[node*2+1]+seg[node*2+2];
        }
        build(&nums,&mut seg,0,0,n-1);
        Self{seg}
    }

    
    fn update(&mut self, index: i32, val: i32) {
        fn change(seg:&mut Vec<i32>,index:usize,val:i32,node:usize,s:usize,e:usize){
            if s==e{
                seg[node]=val;
                return 
            }
            let m=s+(e-s)/2;
            if index<=m{
                change(seg,index,val,node*2+1,s,m);
            }else{
                change(seg,index,val,node*2+2,m+1,e);
            }
            seg[node]=seg[node*2+1]+seg[node*2+2];
        }
        let n=self.seg.len()/4;
        change(&mut self.seg,index as usize,val,0,0,n-1);
    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
         fn range(seg:&Vec<i32>,left:usize,right:usize,node:usize,s:usize,e:usize)->i32{
            if s==left && right==e{
                return  seg[node]
            }
            let m=s+(e-s)/2;
            if right<=m{
                 range(seg,left,right,node*2+1,s,m)
            }else if left>m{
                  range(seg,left,right,node*2+2,m+1,e)
            }else{
                range(seg,left,m,node*2+1,s,m)+range(seg,m+1,right,node*2+2,m+1,e)
            }
            
        }
        let n=self.seg.len()/4;
        range(&self.seg,left as usize,right as usize,0,0,n-1)
    }
}