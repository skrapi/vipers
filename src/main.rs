extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop:: *;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

use std::collections::LinkedList;
use std::iter::FromIterator;

use rand::Rng;

#[derive(Clone, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down
}

struct Size {
    width: u32,
    height: u32,
}

struct Game {
    gl: GlGraphics,
    size: Size,
    snake: Snake,
    food: Food,
}

impl Game {
    fn render(&mut self, arg: &RenderArgs) {


    let _grey: [f32; 4] = [0.788, 0.808, 0.741, 1.0];
    let _light_green: [f32; 4] = [0.698, 0.737, 0.667, 1.0];
    let green: [f32; 4] = [0.514, 0.557, 0.514, 1.0];
    
    self.gl.draw(arg.viewport(), |_c, gl| {
        graphics::clear(green, gl);
    });
    
    self.snake.render(&mut self.gl, arg);

    self.food.render(&mut self.gl, arg);
    }

    fn update(&mut self) {
        self.snake.update();
        
        let snake_head = (*self.snake.body.front().expect("Snake has no body")).clone();
        if (snake_head.0 == self.food.x) && (snake_head.1 == self.food.y) {
            self.food.update();
        }
    }

    fn pressed(&mut self, btn: &Button) {
        let last_direction = self.snake.dir.clone();

        self.snake.dir = match btn {
            &Button::Keyboard(Key::Up)
                if last_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down)
                if last_direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Left)
                if last_direction != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Right)
                if last_direction != Direction::Left => Direction::Right,
            _=> last_direction
        };
    }
}

struct Snake {
    body: LinkedList<(u32, u32)>,
    dir: Direction,
}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let red: [f32; 4] = [0.392, 0.251, 0.243, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self.body
            .iter()
            .map(|&(x,y)| {
                graphics::rectangle::square(
                    (x * 20) as f64, 
                    (y * 20) as f64, 
                    20_f64)
            })
            .collect();


        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            squares.into_iter()
                .for_each(|square| graphics::rectangle(red, square, transform, gl)); 
        });
    }

    fn update (&mut self) {
        let mut new_head = (*self.body.front().expect("Snake has no body")).clone();

        match self.dir {
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,
            Direction::Up => new_head.1 -= 1,
            Direction::Down => new_head.1 += 1,
        }

        self.body.push_front(new_head);

        self.body.pop_back().unwrap();
    }
}

struct Food {
    x: u32,
    y: u32,
}

impl Food {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let brown: [f32; 4] = [0.424, 0.376, 0.380, 1.0];
    
        let square = graphics::rectangle::square(
                    (self.x * 20) as f64, 
                    (self.y * 20) as f64, 
                    20_f64);
    
        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            
            graphics::rectangle(brown, square, transform, gl); 
        });
    }

    fn update(&mut self) {
        // TODO make this not land on snake
        self.x = rand::random::<u8>() as u32;
        self.y = rand::random::<u8>() as u32;
    }
}
fn main() {
    let opengl = OpenGL::V4_5;

    let mut rng = rand::thread_rng();

    let width = 640;
    let height = 480;

    let mut window: GlutinWindow = WindowSettings::new(
        "Vipers",
         [width, height])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        size: Size {
            width,
            height,
        },
        snake: Snake { 
            body: LinkedList::from_iter((vec![(0,0), (0,1)]).into_iter()), 
            dir: Direction::Right 
        },
        food: Food {
            x: rng.gen_range(0, width/20),
            y: rng.gen_range(0, height/20),
        }
    };

    let mut events = Events::new(EventSettings::new()).ups(8);

    while let Some(event) = events.next(&mut window) {

        if let Some(r) = event.render_args() {
            game.render(&r);
        }

        if let Some(_) = event.update_args() {
            game.update();
        }

        if let Some(k) = event.button_args() {
            if k.state == ButtonState::Press {
                if k.button == Button::Keyboard(Key::Escape){
                    break;
                }
                else {
                    game.pressed(&k.button);
                }
            }
        }
    }

}