/*
 * @lc app=leetcode id=1505 lang=rust
 *
 * [1505] Minimum Possible Integer After at Most K Adjacent Swaps On Digits
 */

// @lc code=start
impl Solution {
    pub fn min_integer(num: String, k: i32) -> String {
        let n = num.len();
        let mut tree = vec![0; n + 1];
        let lowbit = |x: i32| -> i32 { x & (-x) };
        let update = |mut x: i32, tree: &mut Vec<i32>| {
            while x < tree.len() as i32 {
                tree[x as usize] += 1;
                x += lowbit(x);
            }
        };
        let query = |mut x: i32, tree: &Vec<i32>| {
            let mut ans = 0;
            while x > 0 {
                ans += tree[x as usize];
                x -= lowbit(x);
            }
            ans
        };
        let query_range = |x: i32, y: i32, tree: &Vec<i32>| -> i32 { query(y,tree) - query(x - 1,tree) };
        let mut pos = vec![Vec::new(); 10];
        let bn = num.as_bytes();
        for i in (0..n).rev() {
            pos[(bn[i] - b'0') as usize].push(i as i32 + 1);
        }
        let mut ans = Vec::new();
        let mut k = k;
        for i in 1..=n {
            for j in 0..10 {
                if pos[j].is_empty() {
                    continue;
                }
                let lastj = pos[j][pos[j].len() - 1];
                let behind = query_range(lastj + 1, n as i32,&tree);
                let dist = lastj + behind - i as i32;
                if dist <= k {
                    update(pos[j].pop().unwrap(), &mut tree);
                    ans.push(b'0' + j as u8);
                    k -= dist;
                    break;
                }
            }
        }
        String::from_utf8(ans).unwrap()
    }
}
// @lc code=end
impl Solution {
    pub fn min_integer(num: String, mut k: i32) -> String {
    
        let mut digit2idx=num.bytes().enumerate().rev().fold(std::collections::HashMap::new(),|mut a,(i,b)|  {a.entry((b-b'0') as i32).or_insert(Vec::new()).push(i as i32);a});
        let n=num.len() as i32;
        let mut tree=vec![0;n as usize*4+1];
        let mut ans=String::new();
        for i in 0..n{
            for x in 0..10{
                if digit2idx.get(&x).unwrap_or(&Vec::new()).is_empty(){
                    continue
                }
                let behind_swaptime=query(0,0,n-1,*digit2idx[&x].last().unwrap(),n-1,&tree ) ;
                let dist=*digit2idx[&x].last().unwrap()-i+behind_swaptime;
                if dist>k{
                    continue
                }
                update(0,0,n-1,*digit2idx[&x].last().unwrap(),1,&mut tree ) ;
                digit2idx.entry(x).and_modify(|v|{v.pop();});
                ans.push((b'0'+x as u8) as char);
                k-=dist;
                break
            }
        }
        fn update(mut i:i32,l:i32,r:i32,id:i32,diff:i32,tree:&mut Vec<i32>){
            if id==l && id==r{
                tree[i as usize]+=diff;
                return 
            }
            let (left ,right)=(i*2+1,i*2+2);
            let mid=(l+r)/2;
            if id<=mid{
                update(left,l,mid,id,diff,tree);
            }else{
                update(right,mid+1,r,id,diff,tree);
            }
            tree[i as usize]=tree[left  as usize]+tree[right  as usize];
        }
        fn query(mut i:i32,l:i32,r:i32,ql:i32,qr:i32,tree:& Vec<i32>)->i32{
            if ql==l && qr==r{
                return tree[i as usize]
            }
            let (left ,right)=(i*2+1,i*2+2);
            let mid=(l+r)/2;
            if qr<=mid{
                query(left,l,mid,ql,qr,tree)
            }else if mid+1<=ql{
                query(right,mid+1,r,ql,qr,tree)
            }else{
                 query(left,l,mid,ql,mid,tree)+query(right,mid+1,r,mid+1,qr,tree)
            }
        }
        
        ans
    }
}