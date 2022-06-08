/*
 * @lc app=leetcode id=2069 lang=rust
 *
 * [2069] Walking Robot Simulation II
 */

// @lc code=start
struct Robot {
    pos: Vec<Vec<i32>>,
    dir: Vec<usize>,
    idx: usize,
    moved: bool,
    dirs: Vec<String>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Robot {
    fn new(width: i32, height: i32) -> Self {
        let mut pos = Vec::new();
        let mut dir = Vec::new();
        for i in 0..width {
            pos.push(vec![i, 0]);
            dir.push(0);
        }
        for i in 1..height {
            pos.push(vec![width - 1, i]);
            dir.push(1);
        }
        for i in (0..width - 1).rev() {
            pos.push(vec![i, height - 1]);
            dir.push(2);
        }
        for i in (1..height - 1).rev() {
            pos.push(vec![0, i]);
            dir.push(3);
        }
        dir[0] = 3;
        Self {
            dirs: ["East", "North", "West", "South"]
                .iter()
                .map(|x| x.to_string())
                .collect(),
            dir,
            pos,
            idx: 0,
            moved: false,
        }
    }

    fn step(&mut self, num: i32) {
        self.moved = true;
        self.idx = (self.idx + num as usize) % self.pos.len();
    }

    fn get_pos(&self) -> Vec<i32> {
        self.pos[self.idx].clone()
    }

    fn get_dir(&self) -> String {
        if !self.moved {
            return "East".to_string();
        }
        self.dirs[self.dir[self.idx]].clone()
    }
}
/**
 * Your Robot object will be instantiated and called as such:
 * let obj = Robot::new(width, height);
 * obj.step(num);
 * let ret_2: Vec<i32> = obj.get_pos();
 * let ret_3: String = obj.get_dir();
 */
// @lc code=end
