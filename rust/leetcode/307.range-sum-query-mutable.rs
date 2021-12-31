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

