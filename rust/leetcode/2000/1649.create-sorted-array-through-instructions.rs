/*
 * @lc app=leetcode id=1649 lang=rust
 *
 * [1649] Create Sorted Array through Instructions
 */

// @lc code=start
impl Solution {
    pub fn create_sorted_array(instructions: Vec<i32>) -> i32 {
        let n = *instructions.iter().max().unwrap();
        let mut tree = vec![0; n as usize + 1];
        let lowbit = |x: i32| x & (-x);
        let update = |mut x: i32, tree: &mut Vec<i32>| {
            while x <= n {
                tree[x as usize] += 1;
                x += lowbit(x);
            }
        };
        let query = |mut x: i32, tree: &Vec<i32>| -> i32 {
            let mut ans = 0;
            while x > 0 {
                ans += tree[x as usize];
                x -= lowbit(x);
            }
            ans
        };
        let mut ans = 0;
        for (i, &x) in instructions.iter().enumerate() {
            let smaller = query(x - 1, &tree);
            let larger = i as i32 - query(x, &tree);
            ans += smaller.min(larger);
            ans %= 1_000_000_007;
            update(x, &mut tree);
        }
        ans
    }
}
// @lc code=end
impl Solution {
    pub fn create_sorted_array(instructions: Vec<i32>) -> i32 {
        let m=(*instructions.iter().max().unwrap()) as usize;
        let mut tree=vec![0;m*4];
        fn update(i:i32,l:i32,r:i32,x:i32,tree:&mut Vec<i32>){
            if l>x || r<x{
                return 
            }
            tree[i as usize]+=1;
            if l==r{
                return
            }
            let mid=(l+r)/2;

            update(i*2+1,l,mid,x,tree);
            update(i*2+2,mid+1,r,x,tree);
        }
        fn query(mut i:i32,l:i32,r:i32,ql:i32,qr:i32,tree:& Vec<i32>)->i32{
             if l>qr || r<ql{
                return 0
            }
            if ql<=l && r<=qr{
                return tree[i as usize]
            }
            let mid=(l+r)/2;

            query(i*2+1,l,mid,ql,qr,tree)+
            query(i*2+2,mid+1,r,ql,qr,tree)
        }
       (instructions.iter().enumerate().fold(0,|mut ans,(i,&x)|{
                ans+=query(0,1,m as i32,1,x-1,&tree).min(query(0,1,m as i32,x+1,m as i32,&tree)) as i64;
               update(0,1,m as i32,x,&mut tree);
               ans
        })
       %1_000_000_007) as _
    }
}