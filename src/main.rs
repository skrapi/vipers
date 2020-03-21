extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop:: *;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};


struct Game {
    gl: GlGraphics,
    snake: Snake,
}

impl Game {
    fn render(&mut self, arg: &RenderArgs) {
    use graphics;


    let GREY: [f32; 4] = [0.788, 0.808, 0.741, 1.0];
    let LIGHT_GREEN: [f32; 4] = [0.698, 0.737, 0.667, 1.0];
    let GREEN: [f32; 4] = [0.514, 0.557, 0.514, 1.0];
    let BROWN: [f32; 4] = [0.424, 0.376, 0.380, 1.0];
    let RED: [f32; 4] = [0.392, 0.251, 0.243, 1.0];

    self.gl.draw(arg.viewport(), |_c, gl| {
        graphics::clear(GREEN, gl);
    });

    }
}

struct Snake {
    pos_x: i32,
    pos_y: i32,
}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs)
}
fn main() {
    let opengl = OpenGL::V4_5;

    let mut window: GlutinWindow = WindowSettings::new(
        "Vipers",
         [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

        let mut game = Game {
            gl: GlGraphics::new(opengl)
        };

        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut window) {

            if let Some(r) = e.render_args() {
                game.render(&r);
            }
        }

}