use piston_window::{rectangle::Shape, types::Color, Context, G2d, Key};
use rand::{thread_rng, Rng};

use crate::{
    snake_snake::snake::{Direction, Snake},
    snake_window::draw::{draw_block, draw_rectangle},
};

/// 食物颜色
const FOOD_COLOR: Color = [255.0, 0.0, 255.0, 1.0];
/// 边框颜色
const BORDER_COLOR: Color = [0.0, 0.5, 0.5, 0.6];

/// 游戏结束颜色
const GAMEOVER_COLOR: Color = [0.9, 0.0, 0.0, 0.5];

/// 移动周期，每过 MOVING_PERIOD 秒移动一次
const MOVING_PERIOD: f64 = 0.3;

/// 游戏主体
pub struct Game {
    /// 蛇的主体
    snake: Snake,
    /// 食物是否存在
    food_exists: bool,
    /// 食物x坐标
    food_x: i32,
    /// 食物y坐标
    food_y: i32,
    /// 游戏的宽
    width: i32,
    /// 游戏的高
    height: i32,
    /// 游戏是否结束
    game_over: bool,
    /// 等待时间
    waiting_time: f64,
    /// 是否暂停
    game_pause: bool,
}

impl Game {
    /// 初始化游戏数据
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            food_exists: true,
            food_x: 6,
            food_y: 4,
            width,
            height,
            game_over: false,
            waiting_time: 0.0,
            game_pause: false,
        }
    }

    /// 对外暴露的控制方法
    pub fn key_pressed(&mut self, key: Key) {
        if key == Key::R {
            self.restart();
        }

        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            Key::P => {
                self.game_pause = !self.game_pause;
                None
            }
            _ => None,
        };

        if let Some(d) = dir {
            // 如果输入方向为当前方向的相反方向，不做任何处理
            if d == self.snake.head_direction().opposite() {
                return;
            }
        }

        self.update_snake(dir);
    }

    /// 检查蛇头是否吃到果子，吃到则增加蛇身长度
    fn check_eating(&mut self) {
        let (head_x, head_y) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }

    /// 对外暴露的游戏绘制
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);
        if self.food_exists {
            draw_block(
                FOOD_COLOR,
                Shape::Round(8.0, 16),
                self.food_x,
                self.food_y,
                con,
                g,
            );
        }

        // 绘制游戏边界
        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 1, 1, self.height - 2, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 1, 1, self.height - 2, con, g);

        // 如果游戏失败 绘制游戏失败画面
        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    //  更新游戏进程
    pub fn update(&mut self, delta_time: f64) {
        // 如果游戏暂停/结束时，不执行操作
        if self.game_over || self.game_pause {
            return;
        }

        self.waiting_time += delta_time;

        if !self.food_exists {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    /// 添加食物
    fn add_food(&mut self) {
        let mut rng = thread_rng();

        let mut new_x = rng.gen_range(1..self.width - 1);
        let mut new_y = rng.gen_range(1..self.height);

        while self.snake.over_tail(new_x, new_y) {
            new_x = rng.gen_range(1..self.width - 1);
            new_y = rng.gen_range(1..self.height - 1);
        }
        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

    /// 检查当前游戏蛇的生存状态，蛇自身碰撞检测、游戏边界碰撞检测
    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.over_tail(next_x, next_y) {
            return false;
        }

        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    /// 更新蛇的数据
    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.game_pause {
            return;
        }
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    /// 重置游戏
    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.add_food();
        self.game_over = false;
        self.game_pause = false;
    }
}
