use rand::Rng;
use rand::seq::SliceRandom;
use std::io::{self, Write};
use std::collections::VecDeque;

const MAZE_WIDTH: usize = 25;  // 增加迷宫大小
const MAZE_HEIGHT: usize = 15;

#[derive(Clone, PartialEq)]
enum Cell {
    Wall,
    Path,
    Player,
    Exit,
    Coin,  // 新增：收集金币
}

struct Game {
    maze: Vec<Vec<Cell>>,
    player_pos: (usize, usize),
    exit_pos: (usize, usize),
    coins: usize,  // 记录收集的金币数
    moves: usize,  // 记录移动步数
}

impl Game {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut maze = vec![vec![Cell::Wall; MAZE_WIDTH]; MAZE_HEIGHT];
        
        // 使用深度优先算法生成迷宫
        fn generate_maze(maze: &mut Vec<Vec<Cell>>, x: usize, y: usize, rng: &mut rand::rngs::ThreadRng) {
            maze[x][y] = Cell::Path;
            
            let dirs = [(0, 2), (2, 0), (0, -2), (-2, 0)];
            let mut dirs: Vec<_> = dirs.iter().collect();
            dirs.shuffle(rng);
            
            for &(dx, dy) in dirs.iter() {
                let new_x = (x as i32 + dx) as usize;
                let new_y = (y as i32 + dy) as usize;
                
                if new_x > 0 && new_x < MAZE_HEIGHT-1 && new_y > 0 && new_y < MAZE_WIDTH-1 
                   && maze[new_x][new_y] == Cell::Wall {
                    maze[(x as i32 + dx/2) as usize][(y as i32 + dy/2) as usize] = Cell::Path;
                    generate_maze(maze, new_x, new_y, rng);
                }
            }
        }
        
        // 从中心开始生成迷宫
        generate_maze(&mut maze, 1, 1, &mut rng);
        
        // 添加一些随机通道，增加路径选择
        for _ in 0..MAZE_WIDTH/3 {
            let x = rng.gen_range(1..MAZE_HEIGHT-1);
            let y = rng.gen_range(1..MAZE_WIDTH-1);
            if maze[x][y] == Cell::Wall {
                maze[x][y] = Cell::Path;
            }
        }
        
        // 放置金币
        let mut coins_placed = 0;
        while coins_placed < MAZE_WIDTH/2 {
            let x = rng.gen_range(1..MAZE_HEIGHT-1);
            let y = rng.gen_range(1..MAZE_WIDTH-1);
            if maze[x][y] == Cell::Path {
                maze[x][y] = Cell::Coin;
                coins_placed += 1;
            }
        }
        
        // 确保起点和终点是路径，并尽量放在对角
        let start_pos = (1, 1);
        let exit_pos = (MAZE_HEIGHT-2, MAZE_WIDTH-2);
        maze[start_pos.0][start_pos.1] = Cell::Player;
        maze[exit_pos.0][exit_pos.1] = Cell::Exit;

        // 确保起点到终点有路可走
        fn find_path(maze: &mut Vec<Vec<Cell>>, start: (usize, usize), end: (usize, usize)) -> bool {
            let mut queue = VecDeque::new();
            let mut visited = vec![vec![false; MAZE_WIDTH]; MAZE_HEIGHT];
            queue.push_back(start);
            visited[start.0][start.1] = true;

            while let Some(pos) = queue.pop_front() {
                if pos == end {
                    return true;
                }

                let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
                for (dx, dy) in dirs.iter() {
                    let new_x = (pos.0 as i32 + dx) as usize;
                    let new_y = (pos.1 as i32 + dy) as usize;
                    
                    if new_x < MAZE_HEIGHT && new_y < MAZE_WIDTH 
                       && !visited[new_x][new_y]
                       && (maze[new_x][new_y] == Cell::Path 
                           || maze[new_x][new_y] == Cell::Exit
                           || maze[new_x][new_y] == Cell::Coin) {
                        queue.push_back((new_x, new_y));
                        visited[new_x][new_y] = true;
                    }
                }
            }
            false
        }

        // 如果没有路径，添加一些通道
        if !find_path(&mut maze, start_pos, exit_pos) {
            let mut current = start_pos;
            while current != exit_pos {
                let dx = if exit_pos.0 > current.0 { 1 } else { -1 };
                let dy = if exit_pos.1 > current.1 { 1 } else { -1 };
                
                if rand::random() {
                    current.0 = ((current.0 as i32 + dx) as usize).min(MAZE_HEIGHT-1);
                } else {
                    current.1 = ((current.1 as i32 + dy) as usize).min(MAZE_WIDTH-1);
                }
                maze[current.0][current.1] = Cell::Path;
            }
        }

        Game {
            maze,
            player_pos: start_pos,
            exit_pos,
            coins: 0,
            moves: 0,
        }
    }

    fn display(&self) {
        clearscreen::clear().expect("Failed to clear screen");
        println!("移动步数: {}  收集金币: {}", self.moves, self.coins);
        println!("┌{}┐", "─".repeat(MAZE_WIDTH));
        for row in &self.maze {
            print!("│");
            for cell in row {
                let symbol = match cell {
                    Cell::Wall => "█",
                    Cell::Path => " ",
                    Cell::Player => "P",
                    Cell::Exit => "E",
                    Cell::Coin => "©",
                };
                print!("{}", symbol);
            }
            println!("│");
        }
        println!("└{}┘", "─".repeat(MAZE_WIDTH));
        println!("\n使用 WASD 移动，Q 退出");
    }

    fn move_player(&mut self, direction: char) -> bool {
        let (dx, dy) = match direction {
            'w' => (-1, 0),
            's' => (1, 0),
            'a' => (0, -1),
            'd' => (0, 1),
            _ => return false,
        };

        let new_x = self.player_pos.0 as i32 + dx;
        let new_y = self.player_pos.1 as i32 + dy;

        if new_x >= 0 && new_x < MAZE_HEIGHT as i32 && 
           new_y >= 0 && new_y < MAZE_WIDTH as i32 {
            let new_x = new_x as usize;
            let new_y = new_y as usize;
            
            if self.maze[new_x][new_y] != Cell::Wall {
                // 检查是否收集到金币
                if self.maze[new_x][new_y] == Cell::Coin {
                    self.coins += 1;
                }
                
                // 移动玩家
                self.maze[self.player_pos.0][self.player_pos.1] = Cell::Path;
                self.maze[new_x][new_y] = Cell::Player;
                self.player_pos = (new_x, new_y);
                self.moves += 1;
            }
        }

        // 检查是否到达终点
        self.player_pos == self.exit_pos
    }
}

fn main() {
    let mut game = Game::new();
    
    loop {
        game.display();
        
        print!("输入移动方向: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            continue;
        }
        
        let command = input.trim().chars().next().unwrap_or('x');
        if command == 'q' {
            println!("游戏结束!");
            break;
        }
        
        if game.move_player(command) {
            println!("\n恭喜你找到出口！");
            break;
        }
    }
}
