/*
 * @lc app=leetcode id=1803 lang=rust
 *
 * [1803] Count Pairs With XOR in a Range
 */

// @lc code=start
impl Solution {
    pub fn count_pairs(nums: Vec<i32>, low: i32, high: i32) -> i32 {
        let n = 20011;
        let mut ne = vec![vec![0; 2]; n * 20];
        let mut cnt = vec![0; n * 15];
        let insert = |x: i32, idx: &mut i32, ne: &mut Vec<Vec<i32>>, cnt: &mut Vec<i32>| {
            let mut p = 0;
            for i in (0..=15).rev() {
                let j = (x >> i & 1) as usize;
                if ne[p][j] == 0 {
                    *idx += 1;
                    ne[p][j] = *idx;
                }
                p = ne[p][j] as usize;
                cnt[p] += 1;
            }
        };
        let query = |x: i32, hi: i32, ne: &Vec<Vec<i32>>, cnt: &Vec<i32>| {
            let mut ans = 0;
            let mut p = 0;
            for i in (0..=15).rev() {
                let j = (x >> i & 1) as usize;
                let k = hi >> i & 1;
                if k > 0 {
                    if ne[p][j] > 0 {
                        ans += cnt[ne[p][j] as usize];
                    }
                    if ne[p][1 - j] == 0 {
                        return ans;
                    }
                    p = ne[p][1 - j] as usize;
                } else {
                    if ne[p][j] == 0 {
                        return ans;
                    }
                    p = ne[p][j] as usize;
                }
            }
            ans += cnt[p];
            ans
        };

        let mut idx = 0;
        let mut ans = 0;
        for &num in &nums {
            insert(num, &mut idx, &mut ne, &mut cnt);
            ans += query(num, high, &ne, &cnt) - query(num, low - 1, &ne, &cnt);
        }
        ans
    }
}
// @lc code=end
impl Solution {
    pub fn count_pairs(nums: Vec<i32>, low: i32, high: i32) -> i32 {
        #[derive(Default)]
        struct Trie{
            children:[Option<Box<Trie>>;2],
            sum:i32,
        }
        let insert=|mut node:&mut Trie,num:i32|{
            for i in (0..15).rev(){
                let bit=((num>>i)&1) as usize;
                node=node.children[bit].get_or_insert(Box::new(Trie::default()));
                node.sum+=1;
            }
        };
        let sum=|mut node:&Trie,num:i32,x:i32|{
            let mut ans=0;
            for i in (0..15).rev(){
                let mut bit=((num>>i)&1) as usize;
                if (x>>i&1)>0{
                         if let Some(child)=&node.children[bit]{
                                    ans+=child.sum;
                        }
                                bit^=1;
                }
                  if let Some(child)=&node.children[bit]{
                                    node=child;
                                }else{
                                    return ans
                                }
                

            }
            ans+=node.sum;
            ans
        };
        let f=|x:i32|{
            let mut ans=0;
            let mut trie=Trie::default();
            for w in nums.windows(2){
                insert(&mut trie,w[0]);
                ans+=sum(&trie,w[1],x);
            }
            ans
        };
        f(high)-f(low-1)
    }
}