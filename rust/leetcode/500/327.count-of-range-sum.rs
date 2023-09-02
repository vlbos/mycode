/*
 * @lc app=leetcode id=327 lang=rust
 *
 * [327] Count of Range Sum
 */

// @lc code=start
impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
       let mut s = 0;
        let mut sum = vec![0];
        for &num in &nums {
            s += num as i64;
            sum.push(s);
        }
        fn count_range_sum_recursive(
            sum: &mut Vec<i64>,
            lower: i64,
            upper: i64,
            left: usize,
            right: usize,
        ) -> i32 {
            if left == right {
                return 0;
            }
            let mid = (left + right) / 2;
            let n1 = count_range_sum_recursive(sum, lower, upper, left, mid);
            let n2 = count_range_sum_recursive(sum, lower, upper, mid + 1, right);
            let mut ans = n1 + n2;
            let mut i = left;
            let (mut l, mut r) = (mid + 1, mid + 1);
            while i <= mid {
                while l <= right && sum[l] - sum[i] < lower {
                    l += 1;
                }
                while r <= right && sum[r] - sum[i] <= upper {
                    r += 1;
                }
                ans += (r - l) as i32;
                i += 1;
            }
            let mut sorted = vec![0; right - left + 1];
            let (mut p1, mut p2) = (left, mid + 1);
            let mut p = 0;
            while p1 <= mid || p2 <= right {
                if p1 > mid {
                    sorted[p] = sum[p2];
                    p2 += 1;
                } else if p2 > right {
                    sorted[p] = sum[p1];
                    p1 += 1;
                } else {
                    if sum[p1] < sum[p2] {
                        sorted[p] = sum[p1];
                        p1 += 1;
                    } else {
                        sorted[p] = sum[p2];
                        p2 += 1;
                    }
                }
                p += 1;
            }
            for (i, &v) in sorted.iter().enumerate() {
                sum[left + i] = v;
            }
            ans
        }
        let n = sum.len();
        count_range_sum_recursive(&mut sum, lower as i64, upper as i64, 0, n - 1)
    }
}
// @lc code=end
impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let mut sum:Vec<i64>=nums.into_iter().scan(0,|mut a,c| {*a+=c as i64;Some(*a)}).collect();
        sum.insert(0,0);
        fn count_range_sum_inner(mut sum: Vec<i64>, lower: i64, upper: i64) -> (Vec<i64>,i32) {
                      

            if sum.len()<2{
                return (sum,0)
            }
            let n=sum.len();
            let sum2=sum.split_off(n/2);
            
            let ((n1,cnt1),(n2,cnt2))=(count_range_sum_inner(sum,lower,upper),count_range_sum_inner(sum2,lower,upper));
            let mut cnt=cnt1+cnt2;
            let (mut l,mut r)=(0,0);
            for &v in &n1{
                while l<n2.len() && n2[l]-v<lower{
                    l+=1;
                }
                while  r<n2.len() && n2[r]-v<=upper{
                    r+=1;
                }
                
                cnt+=(r-l) as  i32;
            }
            let mut ans=Vec::new();
             let (mut i,mut j)=(0,0);
            while i<n1.len()&& j<n2.len(){
                if n1[i]<n2[j]{
                    ans.push(n1[i]);
                    i+=1;
                }else{
                    ans.push(n2[j]);
                    j+=1;
                }
            }
            if i<n1.len(){
                    ans.extend_from_slice(&n1[i..]);
            }
            else if j<n2.len(){
                    ans.extend_from_slice(&n2[j..]);
            }
            (ans,cnt)
        }
        count_range_sum_inner(sum,lower as i64,upper as i64).1 
    }
}



impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let n=nums.len();
        let mut all_nums=vec![0];
        let mut pre_sum=vec![0;n+1];
        for (i,&v) in nums.iter().enumerate(){
            pre_sum[i+1]=pre_sum[i]+v as i64;
            all_nums.extend(vec![pre_sum[i+1],pre_sum[i+1]-lower as i64,pre_sum[i+1]-upper as i64]);
        }
        all_nums.sort();
        all_nums.dedup();
        let kth:std::collections::HashMap<i64,i32>=all_nums.into_iter().enumerate().map(|(i,v)|(v,i as i32+1)).collect();
        let mut tree=vec![0;kth.len()+1];
        let inc=|mut i:i32,tree:&mut Vec<i64>|{
            while i<tree.len() as i32{
                tree[i as usize]+=1;
                i+=-i&i;
            }
        };
        let sum=|mut i:i32,tree:&Vec<i64>|{
            let mut ans=0;
            while i>0{
                ans+=tree[i as usize];
                i&=i-1;
            }
            ans
        };
        let query=|i:i32,j:i32,tree:&Vec<i64>|{
            sum(j,tree)-sum(i-1,tree)
        };
        inc(kth[&0],&mut tree);
        let mut cnt=0;
        for &s in &pre_sum[1..]{
           cnt+=query(kth[&(s-upper as i64)],kth[&(s-lower  as i64)],&tree);
           inc(kth[&s],&mut tree);
        }
        cnt as _
    }
}


impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let (lower,upper)=(lower as i64,upper as i64);
        fn build(left:i32,right:i32)->Option<Box<SegNode>>{
            let mut node=SegNode::new(left,right);
            if left==right{
                return Some(Box::new(node))
            }
            let mid=(left+right)/2;
            node.lchild=build(left,mid);
             node.rchild=build(mid+1,right);
             Some(Box::new(node))
        }
        fn insert(root:&mut Option<Box<SegNode>>,val:i32){
            root.as_mut().unwrap().add+=1;
            if root.as_ref().unwrap().lo==root.as_ref().unwrap().hi{
                return 
            }
            let mid=(root.as_ref().unwrap().lo+root.as_ref().unwrap().hi)/2;
            if val<=mid{
                insert(&mut root.as_mut().unwrap().lchild,val);
            }else{
                insert(&mut root.as_mut().unwrap().rchild,val);
            }
        }
        fn count(root:&Option<Box<SegNode>>,left:i32,right:i32)->i32{
            if right<root.as_ref().unwrap().lo ||root.as_ref().unwrap().hi<left {
                return 0
            }
             if left<=root.as_ref().unwrap().lo &&  right >= root.as_ref().unwrap().hi {
                return root.as_ref().unwrap().add
            }
          
                count(& root.as_ref().unwrap().lchild,left,right)+
                count(& root.as_ref().unwrap().rchild,left,right)
        }
        let mut pre_sum:Vec<i64>=nums.iter().scan(0,|mut a,&c| {*a+=c as i64;Some(*a)}).collect();
        pre_sum.insert(0,0);
        let mut all_numbers:Vec<i64>=pre_sum.iter().map(|&x| vec![x,x-upper,x-lower]).flatten().collect();
        all_numbers.sort();
        all_numbers.dedup();
        let values:std::collections::HashMap<i64,i32>=all_numbers.into_iter().enumerate().map(|(i,v)|(v,i as i32)).collect();
        let mut root=build(0,values.len() as i32-1);
        let mut ans=0;
        for &x in &pre_sum{
                let (left,right)=(values[&(x-upper)],values[&(x-lower)]);
                ans+=count(&root,left,right);
                insert(&mut root,values[&x]);
        }
        ans
    }
}
struct SegNode{
    lo:i32,
    hi:i32,
    add:i32,
    lchild:Option<Box<SegNode>>,
    rchild:Option<Box<SegNode>>,
}
impl SegNode{
    fn new(left:i32,right:i32)->Self{
        Self{lo:left,hi:right,add:0,lchild:None,rchild:None}
    }

}



impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let (lower,upper)=(lower as i64,upper as i64);
        fn insert(root:&mut Option<Box<SegNode>>,val:i64){
            root.as_mut().unwrap().add+=1;
            if root.as_ref().unwrap().lo==root.as_ref().unwrap().hi{
                return 
            }
            let mid=root.as_ref().unwrap().lo+(root.as_ref().unwrap().hi-root.as_ref().unwrap().lo)/2;
            if val<=mid{
                if root.as_mut().unwrap().lchild.is_none(){
                    root.as_mut().unwrap().lchild=Some(Box::new(SegNode::new(root.as_ref().unwrap().lo,mid)));
                }
                insert(&mut root.as_mut().unwrap().lchild,val);
            }else{
                if root.as_mut().unwrap().rchild.is_none(){
                    root.as_mut().unwrap().rchild=Some(Box::new(SegNode::new(mid+1,root.as_ref().unwrap().hi)));
                }
                insert(&mut root.as_mut().unwrap().rchild,val);
            }
        }
        fn count(root:&Option<Box<SegNode>>,left:i64,right:i64)->i32{
            if root.is_none(){
                return 0
            }
            if right<root.as_ref().unwrap().lo ||root.as_ref().unwrap().hi<left {
                return 0
            }
             if left<=root.as_ref().unwrap().lo &&  right >= root.as_ref().unwrap().hi {
                return root.as_ref().unwrap().add
            }
          
                count(& root.as_ref().unwrap().lchild,left,right)+
                count(& root.as_ref().unwrap().rchild,left,right)
        }
        let mut pre_sum:Vec<i64>=nums.iter().scan(0,|mut a,&c| {*a+=c as i64;Some(*a)}).collect();
        pre_sum.insert(0,0);
        let bound=pre_sum.iter().fold(vec![i64::MAX,i64::MIN],|mut a,&x| {a=vec![*[a[0],x,x-upper,x-lower].into_iter().min().unwrap(),*[a[1],x,x-upper,x-lower].into_iter().max().unwrap()];a});
        let mut root=Some(Box::new(SegNode::new(bound[0],bound[1])));
        let mut ans=0;
        for &x in &pre_sum{
                ans+=count(&root,x-upper,x-lower);
                insert(&mut root,x);
        }
        ans
    }
}
struct SegNode{
    lo:i64,
    hi:i64,
    add:i32,
    lchild:Option<Box<SegNode>>,
    rchild:Option<Box<SegNode>>,
}
impl SegNode{
    fn new(left:i64,right:i64)->Self{
        Self{lo:left,hi:right,add:0,lchild:None,rchild:None}
    }

}