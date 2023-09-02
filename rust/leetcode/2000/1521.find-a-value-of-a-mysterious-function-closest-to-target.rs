/*
 * @lc app=leetcode id=1521 lang=rust
 *
 * [1521] Find a Value of a Mysterious Function Closest to Target
 */

// @lc code=start
impl Solution {
    pub fn closest_to_target(arr: Vec<i32>, target: i32) -> i32 {
        let mut ans = (arr[0] - target).abs();
        let mut valid = vec![arr[0]];
        for &num in &arr {
            let mut new_valid = vec![num];
            ans = ans.min((num - target).abs());
            for &prev in &valid {
                new_valid.push(prev & num);
                ans = ans.min(((prev&num) - target).abs());
            }
            new_valid.dedup();
            valid = new_valid;
        }
        ans
    }
}
// @lc code=end
impl Solution {
    pub fn closest_to_target(arr: Vec<i32>, target: i32) -> i32 {
        fn build (i:i32,l:i32,r:i32, arr: &Vec<i32>,tree:&mut Vec<i32>){
            if l==r{
                tree[i as usize]=if l<arr.len() as i32{arr[l as usize]}else{0};
                return 
            }
            let mid=(r+l)/2;
            build(i*2,l,mid,arr,tree);
            build(i*2+1,mid+1,r,arr,tree);
            tree[i as usize]= tree[i as usize*2]& tree[i as usize*2+1];
        }
        fn ask (i:i32,l:i32,r:i32,ll:i32,rr:i32,tree:&Vec<i32>)->i32{
            if l==ll && rr ==r{
                return tree[i as usize]
            }
            let mid=(r+l)/2;
            if rr<=mid{
                ask(i*2,l,mid,ll,rr,tree)
            }else if mid<ll{
                ask(i*2+1,mid+1,r,ll,rr,tree)
            }else{
                ask(i*2,l,mid,ll,mid,tree)&ask(i*2+1,mid+1,r,mid+1,rr,tree)
            }
        }
        let n=arr.len() as i32;
        let mut tree=vec![0;arr.len() *4+1];
        build(1,0,n-1,&arr,&mut tree);
        let (mut l,mut r)=(0,0);
        let mut ans=i32::MAX;
        while r<n{
            let t=ask(1,0,n-1,l,r,&tree);
            if t<target{
                l+=1;
                r=r.max(l);
            }else{
                r+=1;
            }
            ans=ans.min((t-target).abs());
        }
        ans
    }
}