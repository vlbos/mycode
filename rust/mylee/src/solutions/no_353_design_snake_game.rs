// 353\. Design Snake Game
// =======================

// Design a [Snake game](https://en.wikipedia.org/wiki/Snake_(video_game))
// that is played on a device with screen size = _width_ x _height_.
// [Play the game online](http://patorjk.com/games/snake/) if you are not familiar with the game.

// The snake is initially positioned at the top left corner (0,0) with length = 1 unit.

// You are given a list of food's positions in row-column order. When a snake eats the food,
//  its length and the game's score both increase by 1.

// Each food appears one by one on the screen. For example, the second food will not appear until the first food was eaten by the snake.

// When a food does appear on the screen, it is guaranteed that it will not appear on a block occupied by the snake.

// **Example:**

// Given width = 3, height = 2, and food = \[\[1,2\],\[0,1\]\].

// Snake snake = new Snake(width, height, food);

// Initially the snake appears at position (0,0) and the food at (1,2).

// |S| | |
// | | |F|

// snake.move("R"); -> Returns 0

// | |S| |
// | | |F|

// snake.move("D"); -> Returns 0

// | | | |
// | |S|F|

// snake.move("R"); -> Returns 1 (Snake eats the first food and right after that, the second food appears at (0,1) )

// | |F| |
// | |S|S|

// snake.move("U"); -> Returns 1

// | |F|S|
// | | |S|

// snake.move("L"); -> Returns 2 (Snake eats the second food)

// | |S|S|
// | | |S|

// snake.move("U"); -> Returns -1 (Game over because snake collides with border)

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google) [Salesforce](https://leetcode.ca/tags/#Salesforce) [Uber](https://leetcode.ca/tags/#Uber) [Zillow](https://leetcode.ca/tags/#Zillow)

// @lc code=start
use std::collections::HashSet;
use std::collections::VecDeque;

// const UP: (&str, isize, isize) = ("U", 0, -1);
// const DOWN: (&str, isize, isize) = ("D", 0, 1);
// const LEFT: (&str, isize, isize) = ("L", -1, 0);
// const RIGHT: (&str, isize, isize) = ("R", 1, 0);

// const DIRECTIONS: [(&str, isize, isize); 4] = [UP, DOWN, LEFT, RIGHT];

#[derive(Debug)]
struct SnakeGame {
    // snake: VecDeque<(isize, isize)>,
    // body: HashSet<(isize, isize)>,
    // foods: Vec<(isize, isize)>,
    // width: isize,
    // height: isize,
    snake: VecDeque<Vec<i32>>,
    s: HashSet<Vec<i32>>,
    food: Vec<Vec<i32>>,
    width: i32,
    height: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SnakeGame {
    /** Initialize your data structure here.
    @param width - screen width
    @param height - screen height
    @param food - A list of food positions
    E.g food = [[1,1], [1,0]] means the first food is positioned at [1,1], the second is at [1,0]. */
    pub fn new(width: i32, height: i32, food: Vec<Vec<i32>>) -> Self {
        // SnakeGame {
        //     snake: {
        //         let mut res = VecDeque::new();
        //         res.push_back((0, 0));
        //         res
        //     },
        //     body: HashSet::new(),
        //     foods: food
        //         .into_iter()
        //         .map(|f| (f[1] as isize, f[0] as isize))
        //         .rev()
        //         .collect(),
        //     width: width as isize,
        //     height: height as isize,
        // }
        let mut food = food;
        food.reverse();
        Self {
            width,
            height,
            food,
            snake: VecDeque::from([vec![0, 0]]),
            s: HashSet::from([vec![0, 0]]),
        }
    }

    /** Moves the snake.
    @param direction - 'U' = Up, 'L' = Left, 'R' = Right, 'D' = Down
    @return The game's score after the move. Return -1 if game over.
    Game over when snake crosses the screen boundary or bites its body. */
    pub fn make_a_move(&mut self, direction: String) -> i32 {
        let front = self.snake.front().unwrap();
        let (mut x, mut y) = (front[0], front[1]);
        match direction.as_str() {
            "U" => x -= 1,
            "D" => x += 1,
            "L" => y -= 1,
            _ => y += 1,
        }
        if x < 0 || x >= self.height || y < 0 || y >= self.width {
            return -1;
        }
        if !self.food.is_empty()
            && self.food[self.food.len() - 1][0] == x
            && self.food[self.food.len() - 1][1] == y
        {
            self.food.pop();
        } else {
            let l = self.snake.pop_back().unwrap();
            self.s.remove(&l);
        }
        if self.s.contains(&vec![x, y]) {
            return -1;
        }
        self.snake.push_front(vec![x, y]);
        self.s.insert(vec![x, y]);
        self.s.len() as i32 - 1
        // let head = self.head();
        // self.body.insert(head);
        // let mut next_head: (isize, isize) = (0, 0);
        // for d in &DIRECTIONS {
        //     if direction == d.0 {
        //         next_head = (head.0 + d.1, head.1 + d.2);
        //         break;
        //     }
        // }
        // let mut will_eat = false;
        // if next_head.0 < 0
        //     || next_head.1 < 0
        //     || next_head.0 >= self.width
        //     || next_head.1 >= self.height
        // {
        //     return -1;
        // } else if let Some(f) = self.foods.last() {
        //     if f.0 == next_head.0 && f.1 == next_head.1 {
        //         will_eat = true;
        //     }
        // }
        // match self.snake.pop_back() {
        //     Some(tail_end) => {
        //         self.body.remove(&tail_end);
        //         if will_eat {
        //             self.foods.pop();
        //             self.snake.push_back(tail_end);
        //             self.body.insert(tail_end);
        //         }
        //     }
        //     None => return -1,
        // }
        // if self.body.contains(&next_head) {
        //     return -1;
        // }
        // self.snake.push_front(next_head);
        // (self.snake.len() - 1) as i32
    }
    // fn head(&self) -> (isize, isize) {
    //     self.snake.get(0).unwrap().clone()
    // }
}

// @lc code=end

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_snake_game() {
        let mut snake_game = SnakeGame::new(3, 2, vec![vec![1, 2], vec![0, 1]]);
        let directions = ["R", "D", "R", "U", "L", "U"];
        let result = [0, 0, 1, 1, 2, -1];
        for i in 0..directions.len() {
            assert_eq!(
                snake_game.make_a_move(String::from(directions[i])),
                result[i]
            );
        }
    }
}
