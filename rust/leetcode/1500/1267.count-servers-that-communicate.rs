/*
 * @lc app=leetcode id=1267 lang=rust
 *
 * [1267] Count Servers that Communicate
 */

// @lc code=start
impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let mut row_cnt = vec![0; grid.len()];
        let mut col_cnt = vec![0; grid[0].len()];
        for (i, r) in grid.iter().enumerate() {
            for (j, &c) in r.iter().enumerate() {
                if c == 1 {
                    row_cnt[i] += 1;
                    col_cnt[j] += 1;
                }
            }
        }
        let mut ans = 0;
        for (i, r) in grid.iter().enumerate() {
            for (j, &c) in r.iter().enumerate() {
                if c == 1 && (row_cnt[i] > 1 || col_cnt[j] > 1) {
                    ans += 1;
                }
            }
        }
        ans
    }
}
// @lc code=end
impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let (m,n)=(grid.len(),grid[0].len());
        let mn=m+n;
        let mut parent=vec![-1;mn];
        let unite=|x:usize,y:usize,parent:&mut Vec<i32>|{
            if x==y{
                return
            }
            let (x,y) =if parent[x]>parent[y]{(y,x)}else{(x,y)};
            parent[x]+=parent[y];
            parent[y]=x as i32;
        };
        fn find(x:usize,parent:&mut Vec<i32>)->i32{
            if parent[x]<0{
                return x as _
            }
        
            parent[x]=find(parent[x] as usize,parent);
            parent[x]
        }
        let mut cnt=0;
        for (i,row) in grid.iter().enumerate(){
            for (j,&v) in row.iter().enumerate(){
                if v==1{
                    unite(find(i,&mut parent) as  usize,find(m+j,&mut parent) as usize,&mut parent);
                    cnt+=1;
                }
            }
        }
        cnt-parent.iter().filter(|&i| *i==-2).count() as i32
    }
}