/*
 * @lc app=leetcode id=1622 lang=rust
 *
 * [1622] Fancy Sequence
 */

// @lc code=start
struct Fancy {
    v: Vec<i32>,
    a: i32,
    b: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
const p: i64 = 1_000_000_007;
impl Fancy {
    fn new() -> Self {
        Self {
            v: Vec::new(),
            a: 1,
            b: 0,
        }
    }

    fn append(&mut self, val: i32) {
        let quick_mul = |x: i32, mut y: i64| {
            let mut ans = 1;
            let mut cur = x;
            while y > 0 {
                if y & 1 > 0 {
                    ans = ((ans as i64) * (cur as i64) % p) as i32;
                }
                cur = ((cur as i64) * (cur as i64) % p) as i32;
                y >>= 1;
            }
            ans
        };
        let inv = |x: i32| quick_mul(x, p - 2) as i64;
        self.v
            .push(((((val - self.b) as i64) + p) % p * inv(self.a) % p) as i32);
    }

    fn add_all(&mut self, inc: i32) {
        self.b = (((self.b as i64)+ inc as i64)  % p) as i32;
    }

    fn mult_all(&mut self, m: i32) {
        self.a = ((self.a as i64) * (m as i64) % p) as i32;
        self.b = ((self.b as i64) * (m as i64) % p) as i32;
    }

    fn get_index(&self, idx: i32) -> i32 {
        if idx >= self.v.len() as i32 {
            return -1;
        }
        ((((self.a as i64) * (self.v[idx as usize] as i64)) % p + self.b as i64) % p) as i32
    }
}
/**
 * Your Fancy object will be instantiated and called as such:
 * let obj = Fancy::new();
 * obj.append(val);
 * obj.add_all(inc);
 * obj.mult_all(m);
 * let ret_4: i32 = obj.get_index(idx);
 */
// @lc code=end


struct Fancy {
tree:Vec<i64>,
add:Vec<i64>,
mul:Vec<i64>,
n:i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Fancy {

    fn new() -> Self {
        Self{tree:vec![0;400000],add:vec![0;400000],mul:vec![0;400000],n:-1}
    }
    
    fn append(&mut self, val: i32) {
        self.n+=1;
        add_update(0,0,self.tree.len() as i32/4-1,self.n,self.n,val,&mut self.tree,&mut self.add,&mut self.mul);
    }
    
    fn add_all(&mut self, inc: i32) {
        if self.n>=0{
            add_update(0,0,self.tree.len() as i32/4-1,0,self.n,inc,&mut self.tree,&mut self.add,&mut self.mul);
        }
    }
    
    fn mult_all(&mut self, m: i32) {
        fn mul_update(i:i32,l:i32,r:i32,ul:i32,ur:i32,val:i32,tree:&mut Vec<i64>,add:&mut Vec<i64>,mul:&mut Vec<i64>){
    let i=i as usize;
    if ul<=l && r<=ur{
        tree[i]=(tree[i]*val as i64) %1_000_000_007;
        mul[i]=(mul[i]*val as i64)%1_000_000_007;
        add[i]=(add[i]*val as i64)%1_000_000_007;
        return 
    }
            let (left,right)=(i*2+1,i*2+2);
    if add[i]!=0 || mul[i] !=1
    {
    tree[left]=( tree[left]*mul[i]+add[i])%1_000_000_007;
    tree[right]=( tree[right]*mul[i]+add[i])%1_000_000_007;
    mul[left]=(mul[left]*mul[i])%1_000_000_007;
    mul[right]=(mul[right]*mul[i])%1_000_000_007;
    add[left]=( add[left]*mul[i]+add[i])%1_000_000_007;
    add[right]=( add[right]*mul[i]+add[i])%1_000_000_007;
    mul[i]=1;
    add[i]=0;
    }
    let mid=(l+r)/2;
    if ul<=mid{
        mul_update(left as i32,l,mid,ul,ur,val,tree,add,mul);
    }
    if mid<ur{
        mul_update(right  as i32,mid+1,r,ul,ur,val,tree,add,mul);
    }
    tree[i]=(tree[left]+tree[right])%1_000_000_007;
        }
        if self.n>=0{
            mul_update(0,0,self.tree.len() as i32/4-1,0,self.n,m,&mut self.tree,&mut self.add,&mut self.mul);
        }
    }
    
    fn get_index(&mut self, idx: i32) -> i32 {
        fn query(i:i32,l:i32,r:i32,ul:i32,ur:i32,tree:&mut Vec<i64>,add:&mut Vec<i64>,mul:&mut Vec<i64>)->i64{
            if ul>r|| ur<l{
                return 0
            }
    let i=i as usize;
    if ul<=l && r<=ur{
        return  tree[i]
    }
            let (left,right)=(i*2+1,i*2+2);
    if add[i]!=0 || mul[i] !=1
    {
    tree[left]=( tree[left]*mul[i]+add[i])%1_000_000_007;
    tree[right]=( tree[right]*mul[i]+add[i])%1_000_000_007;
    mul[left]=(mul[left]*mul[i])%1_000_000_007;
    mul[right]=(mul[right]*mul[i])%1_000_000_007;
    add[left]=( add[left]*mul[i]+add[i])%1_000_000_007;
    add[right]=( add[right]*mul[i]+add[i])%1_000_000_007;
    mul[i]=1;
    add[i]=0;
    }
    let mid=(l+r)/2;
        (query(left  as i32,l,mid,ul,ur,tree,add,mul)+
        query(right  as i32,mid+1,r,ul,ur,tree,add,mul))%1_000_000_007
        }
        if idx>self.n{-1}else{
           query(0,0,self.tree.len() as i32/4-1,idx,idx,&mut self.tree,&mut self.add,&mut self.mul) as _
        }
    }
}

fn add_update(i:i32,l:i32,r:i32,ul:i32,ur:i32,val:i32,tree:&mut Vec<i64>,add:&mut Vec<i64>,mul:&mut Vec<i64>){
    let i=i as usize;
    if ul<=l && r<=ur{
        tree[i]=(tree[i]+(r-l+1) as i64*val as i64) %1_000_000_007;
        add[i]=(add[i]+val as i64)%1_000_000_007;
        return 
    }
            let (left,right)=(i*2+1,i*2+2);
    if add[i]!=0 || mul[i] !=1
    {
    tree[left]=( tree[left]*mul[i]+add[i])%1_000_000_007;
    tree[right]=( tree[right]*mul[i]+add[i])%1_000_000_007;
    mul[left]=(mul[left]*mul[i])%1_000_000_007;
    mul[right]=(mul[right]*mul[i])%1_000_000_007;
    add[left]=( add[left]*mul[i]+add[i])%1_000_000_007;
    add[right]=( add[right]*mul[i]+add[i])%1_000_000_007;
    mul[i]=1;
    add[i]=0;
    }
    let mid=(l+r)/2;
    if ul<=mid{
        add_update(left  as i32,l,mid,ul,ur,val,tree,add,mul);
    }
    if mid<ur{
        add_update(right  as i32,mid+1,r,ul,ur,val,tree,add,mul);
    }
    tree[i]=(tree[left]+tree[right])%1_000_000_007;
}


/**
 * Your Fancy object will be instantiated and called as such:
 * let obj = Fancy::new();
 * obj.append(val);
 * obj.add_all(inc);
 * obj.mult_all(m);
 * let ret_4: i32 = obj.get_index(idx);
 */