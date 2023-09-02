/*
 * @lc app=leetcode id=1728 lang=rust
 *
 * [1728] Cat and Mouse II
 */

// @lc code=start
impl Solution {
    pub fn can_mouse_win(grid: Vec<String>, cat_jump: i32, mouse_jump: i32) -> bool {
        let (rows, cols) = (grid.len(), grid[0].len());
        let get_pos = |row: usize, col: usize| row * cols + col;
        let (mut start_cat, mut start_mouse, mut food) = (0, 0, 0);
        for (i, row) in grid.iter().enumerate() {
            for (j, ch) in row.chars().enumerate() {
                match ch {
                    'C' => start_cat = get_pos(i, j),
                    'M' => start_mouse = get_pos(i, j),
                    'F' => food = get_pos(i, j),
                    _ => {}
                }
            }
        }
        let total = rows * cols;
        let mut degrees = vec![vec![vec![0, 0]; total]; total];
        let dirs = [0, 1, 0, -1, 0];
        for mouse in 0..total {
            let (mouse_row, mouse_col) = (mouse / cols, mouse % cols);
            if grid[mouse_row].as_bytes()[mouse_col] == b'#' {
                continue;
            }
            for cat in 0..total {
                let (cat_row, cat_col) = (cat / cols, cat % cols);
                if grid[cat_row].as_bytes()[cat_col] == b'#' {
                    continue;
                }
                degrees[mouse][cat][0] += 1;
                degrees[mouse][cat][1] += 1;
                for i in 0..dirs.len() - 1 {
                    let (dx, dy) = (dirs[i], dirs[i + 1]);
                    let (mut row, mut col, mut jump) =
                        (mouse_row as i32 + dx, mouse_col as i32 + dy, 1);
                    while row >= 0
                        && row < rows as i32
                        && col >= 0
                        && col < cols as i32
                        && grid[row as usize].as_bytes()[col as usize] != b'#'
                        && jump <= mouse_jump
                    {
                        let next_mouse = get_pos(row as usize, col as usize);
                        let next_cat = get_pos(cat_row, cat_col);
                        degrees[next_mouse][next_cat][0] += 1;
                        row += dx;
                        col += dy;
                        jump += 1;
                    }
                    let (mut row, mut col, mut jump) =
                        (cat_row as i32 + dx, cat_col as i32 + dy, 1);
                    while row >= 0
                        && row < rows as i32
                        && col >= 0
                        && col < cols as i32
                        && grid[row as usize].as_bytes()[col as usize] != b'#'
                        && jump <= cat_jump
                    {
                        let next_mouse = get_pos(mouse_row, mouse_col);
                        let next_cat = get_pos(row as usize, col as usize);
                        degrees[next_mouse][next_cat][1] += 1;
                        row += dx;
                        col += dy;
                        jump += 1;
                    }
                }
            }
        }
        let mut results = vec![vec![vec![vec![0, 0], vec![0, 0]]; total]; total];
        let mut q = std::collections::VecDeque::new();
        for pos in 0..total {
            let (row, col) = (pos / cols, pos % cols);
            if grid[row].as_bytes()[col] == b'#' {
                continue;
            }
            results[pos][pos][0][0] = 2;
            results[pos][pos][0][1] = 0;
            results[pos][pos][1][0] = 2;
            results[pos][pos][1][1] = 0;
            q.push_back((pos, pos, 0));
            q.push_back((pos, pos, 1));
        }
        for mouse in 0..total {
            let (row, col) = (mouse / cols, mouse % cols);
            if grid[row].as_bytes()[col] == b'#' || mouse == food {
                continue;
            }
            results[mouse][food][0][0] = 2;
            results[mouse][food][0][1] = 0;
            results[mouse][food][1][0] = 2;
            results[mouse][food][1][1] = 0;
            q.push_back((mouse, food, 0));
            q.push_back((mouse, food, 1));
        }
        for cat in 0..total {
            let (row, col) = (cat / cols, cat % cols);
            if grid[row].as_bytes()[col] == b'#' || cat == food {
                continue;
            }
            results[food][cat][0][0] = 1;
            results[food][cat][0][1] = 0;
            results[food][cat][1][0] = 1;
            results[food][cat][1][1] = 0;
            q.push_back((food, cat, 0));
            q.push_back((food, cat, 1));
        }
        let get_prev_states = |mouse: i32, cat: i32, turn: i32| {
            let cols = cols as i32;
            let (mouse_row, mouse_col) = (mouse / cols, mouse % cols);
            let (cat_row, cat_col) = (cat / cols, cat % cols);
            let (prev_turn, max_jump, start_row, start_col) = if turn == 0 {
                (1, cat_jump, cat_row, cat_col)
            } else {
                (0, mouse_jump, mouse_row, mouse_col)
            };
            let mut prev_states = vec![(mouse as usize, cat as usize, prev_turn)];
            for i in 0..dirs.len() - 1 {
                let (dx, dy) = (dirs[i], dirs[i + 1]);
                let (mut row, mut col, mut jump) =
                    (start_row as i32 + dx, start_col as i32 + dy, 1);
                while row >= 0
                    && row < rows as i32
                    && col >= 0
                    && col < cols as i32
                    && grid[row as usize].as_bytes()[col as usize] != b'#'
                    && jump <= max_jump
                {
                    let (prev_mouse_row, prev_mouse_col, prev_cat_row, prev_cat_col) =
                        if prev_turn == 0 {
                            (row, col, cat_row, cat_col)
                        } else {
                            (mouse_row, mouse_col, row, col)
                        };
                    let prev_mouse = get_pos(prev_mouse_row as usize, prev_mouse_col as usize);
                    let prev_cat = get_pos(prev_cat_row as usize, prev_cat_col as usize);
                    prev_states.push((prev_mouse, prev_cat, prev_turn));
                    row += dx;
                    col += dy;
                    jump += 1;
                }
            }
            prev_states
        };
        while let Some((mouse, cat, turn)) = q.pop_front() {
            let result = results[mouse][cat][turn][0];
            let moves = results[mouse][cat][turn][1];
            for (prev_mouse, prev_cat, prev_turn) in get_prev_states(mouse as i32, cat as i32, turn as i32)
            {
                if results[prev_mouse][prev_cat][prev_turn][0] != 0 {
                    continue;
                }
                if result == 1 && prev_turn == 0 || result == 2 && prev_turn == 1 {
                    results[prev_mouse][prev_cat][prev_turn][0] = result;
                    results[prev_mouse][prev_cat][prev_turn][1] = moves + 1;
                    q.push_back((prev_mouse, prev_cat, prev_turn));
                } else {
                    degrees[prev_mouse][prev_cat][prev_turn] -= 1;
                    if degrees[prev_mouse][prev_cat][prev_turn] == 0 {
                        let lose_result = if prev_turn == 0 { 2 } else { 1 };
                        results[prev_mouse][prev_cat][prev_turn][0] = lose_result;
                        results[prev_mouse][prev_cat][prev_turn][1] = moves + 1;
                        q.push_back((prev_mouse, prev_cat, prev_turn));
                    }
                }
            }
        }
        results[start_mouse][start_cat][0][0] == 1 && results[start_mouse][start_cat][0][1] <= 1000
    }
}
// @lc code=end
impl Solution {
    pub fn can_mouse_win(grid: Vec<String>, cat_jump: i32, mouse_jump: i32) -> bool {
        let mut xy=vec![0;6];
        for (i,row) in grid.iter().enumerate(){
            for (j,c) in row.chars().enumerate(){
                if c=='M'{
                    xy[0]=i;
                    xy[1]=j;
                }else if c=='C'{
                    xy[2]=i;
                    xy[3]=j;
                }else if c=='F'{
                    xy[4]=i;
                    xy[5]=j;
                }
            }
        }
        fn dfs(x:usize,y:usize,p:usize,q:usize,k:usize,tx:usize,ty:usize,f:&mut Vec<Vec<i32>>,grid: &Vec<String>, cat_jump: i32, mouse_jump: i32)->i32{
            let state=(x<<9)|(y<<6)|(p<<3)|q;
            if k==999{
                f[state][k]=1;
                return f[state][k]
            }
            if x==p && y==q{
                f[state][k]=1;
                return f[state][k]
            }
            if x==tx && y==ty{
                f[state][k]=0;
                return f[state][k]
            }
             if tx==p && ty==q{
                f[state][k]=1;
                return f[state][k]
            }
            if f[state][k]!=-1{
                return f[state][k]
            }
            let (m,n)=(grid.len() as i32,grid[0].len() as i32);
            if k%2==0{
                for d in [0,1,0,-1,0].windows(2){
                    for i in 0..=mouse_jump{
                        let (nx,ny)=(x as i32+d[0]*i,y as i32+d[1]*i);
                        if nx<0||nx>=m||ny<0||ny>=n{
                            break
                        }
                        let (nx,ny)=(nx as usize,ny as usize);
                        if grid[nx].as_bytes()[ny]==b'#'{
                            break
                        }
                        if dfs(nx,ny,p,q,k+1,tx,ty,f,grid,cat_jump,mouse_jump)==0{
                             f[state][k]=0;
                            return f[state][k]
                        }
                    }
                }
                  f[state][k]=1;
                return f[state][k]
            }
             for d in [0,1,0,-1,0].windows(2){
                    for i in 0..=cat_jump{
                        let (np,nq)=(p as i32+d[0]*i,q as i32+d[1]*i);
                        if np<0||np>=m||nq<0||nq>=n{
                            break
                        }
                        let (np,nq)=(np as usize,nq as usize);
                        if grid[np].as_bytes()[nq]==b'#'{
                            break
                        }
                        if dfs(x,y,np,nq,k+1,tx,ty,f,grid,cat_jump,mouse_jump)==1{
                             f[state][k]=1;
                            return f[state][k]
                        }
                    }
                }
                  f[state][k]=0;
              f[state][k]
        }
        let mut f=vec![vec![-1;1000];1<<12];
        dfs(xy[0],xy[1],xy[2],xy[3],0,xy[4],xy[5],&mut f,&grid,cat_jump,mouse_jump)==0
    }
}