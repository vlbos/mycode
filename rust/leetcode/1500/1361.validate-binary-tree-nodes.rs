/*
 * @lc app=leetcode id=1361 lang=rust
 *
 * [1361] Validate Binary Tree Nodes
 */

// @lc code=start
impl Solution {
    pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
       let n = n as usize;
        let mut indeg = vec![0; n];
        for &i in left_child.iter().chain(right_child.iter()) {
            if i != -1 {
                indeg[i as usize] += 1;
            }
        }
        let r = indeg.iter().position(|x| *x == 0);
        let mut q = std::collections::VecDeque::<usize>::new();
        let mut seen = vec![false; n];

        if let Some(i) = r {
            q.push_back(i);
            seen[i] = true;
        } else {
            return false;
        }

        while let Some(i) = q.pop_front() {
            let (l, r) = (left_child[i], right_child[i]);
            if l != -1 {
                if seen[l as usize] {
                    return false;
                }
                q.push_back(l as usize);
                seen[l as usize] = true;
            }
            if r != -1  {
                if seen[r as usize] {
                    return false;
                }
                q.push_back(r as usize);
                seen[r as usize] = true;
            }
        }
        seen.iter().all(|x| *x)
    }
}
// @lc code=end
impl Solution {
    pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
        let mut parent:Vec<i32>=(0..n).collect();
        let mut father=vec![-1;n as usize];
        fn find(x:i32,parent:&mut Vec<i32>)->i32{
            let px=parent[x as usize];
            if px!=x{
                parent[x as usize]=find(px,parent);
            }
            parent[x as usize]
        }
        let unite=|x:i32,y:i32,parent:&mut Vec<i32>,father:&mut Vec<i32>,cnt:&mut i32|{
                if y==-1{
                    return true
                }
                if father[y as usize]!=-1{
                    return false
                }
                let (px,py)=(find(x,parent),find(y,parent));
                if px==py{
                    return false
                }
                parent[px as usize]=py;
                father[y as usize]=x;
                *cnt-=1;
                true
        };
        let mut cnt=n;
        for (i,(l,r)) in left_child.into_iter().zip(right_child).enumerate(){
            if l==r&& l!=-1{
                return false
            }
            let i=i as i32;
            if !unite(i,l,&mut parent,&mut father,&mut cnt)||!unite(i,r,&mut parent,&mut father,&mut cnt){
                return false
            }
        }
        cnt==1
    }
}