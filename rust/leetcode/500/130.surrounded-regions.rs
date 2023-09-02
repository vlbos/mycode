/*
 * @lc app=leetcode id=130 lang=rust
 *
 * [130] Surrounded Regions
 */

// @lc code=start
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        fn dfs(board: &mut Vec<Vec<char>>, i: usize, j: usize) {
            if board[i][j] != 'O' {
                return;
            }
            board[i][j] = 'A';
            if j + 1 < board[i].len() {
                dfs(board, i, j + 1);
            }
            if j > 0 {
                dfs(board, i, j - 1);
            }

            if i + 1 < board.len() {
                dfs(board, i + 1, j);
            }
            if i > 0 {
                dfs(board, i - 1, j);
            }
        }
        let n = board.len();
        let m = board[0].len();
        for i in 0..n {
            if board[i][0] == 'O' {
                board[i][0] = 'A';
                if m > 1 {
                    dfs(board, i, 1);
                }
            }
            if board[i][m - 1] == 'O' {
                board[i][m - 1] = 'A';
                if m > 1 {
                    dfs(board, i, m - 2);
                }
            }
        }
        for i in 0..m {
            if board[0][i] == 'O' {
                board[0][i] = 'A';
                if n > 1 {
                    dfs(board, 1, i);
                }
            }
            if board[n - 1][i] == 'O' {
                board[n - 1][i] = 'A';
                if n > 1 {
                    dfs(board, n - 2, i);
                }
            }
        }

        for i in 0..n {
            for j in 0..m {
                if board[i][j] == 'A' {
                    board[i][j] = 'O';
                } else if board[i][j] == 'O' {
                    board[i][j] = 'X';
                }
            }
        }
    }
}
// @lc code=end
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let (m,n)=(board.len(),board[0].len());
        let mn=m*n;
        let mut parent:Vec<usize>=(0..mn+1).collect();
        fn find(x:usize,parent:&mut Vec<usize>)->usize{
            let px=parent[x];
            if px!=x{
                parent[x]=find(px,parent);
            }
            parent[x]
        }
        let unite=|x:usize,y:usize,parent:&mut Vec<usize>|{
            let (px,py)=(find(x,parent),find(y,parent));
            if px==py{
                return
            }
            parent[px]=py;
        };
        for i in 0..m{
            for j in 0..n{
                if board[i][j]=='X'{
                    continue
                }
                if i==0||i==m-1||j==0||j==n-1{
                    unite(i*n+j,mn,&mut parent);
                    continue
                }
                for d in [0,1,0,-1,0].windows(2){
                    let (ni,nj)=(i as i32+d[0],j as i32+d[1]);
                     if ni<0||ni==m as i32 ||nj<0||nj==n as i32{
                        continue
                     }
                     if board[ni as usize][nj as usize]=='O'{
                        unite(i*n+j,ni as usize*n+nj as usize,&mut parent);
                     }
                }
            }
        }
        let pmn=find(mn,&mut parent);
                for i in 0..m{
            for j in 0..n{
                board[i][j]=if find(i*n+j,&mut parent)==pmn{
                        'O'
                }else{'X'};
            }
                }
    }
}