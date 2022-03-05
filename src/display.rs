extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent};
use piston::window::WindowSettings;

use chip_8::CPU;

pub struct App {
    gl: GlGraphics,
}

impl App {
    fn render(&mut self, args: &RenderArgs, screen: &[[bool; 64]; 32]) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let mut squares: Vec<types::Rectangle> = vec![];

        for (row_ind, row) in screen.iter().enumerate() {
            for (col_ind, col) in row.iter().enumerate() {
                if *col {
                    let square = rectangle::square((col_ind*12 + 4) as f64, (row_ind*12 + 4) as f64, 10.0);
                    squares.push(square);
                }
            }
        }

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            for square in squares {
                let transform = c.transform;
                rectangle(GREEN, square, transform, gl);
            }
        });
    }

}

pub struct Game {
    cpu: CPU,
}

impl Game {
    pub fn new(cpu: CPU) -> Game {
        Game { cpu }
    }

    pub fn run(&mut self) {
        let opengl = OpenGL::V3_2;

        let mut window: Window = WindowSettings::new("CHIP-8", [800, 600])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        let mut screen = [[false; 64]; 32];

        let mut app = App {
            gl: GlGraphics::new(opengl),
        };

        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut window) {
            if let None = self.cpu.run(&mut screen) {
                break
            }
            
            if let Some(args) = e.render_args() {
                app.render(&args, &screen);
            }
            
        }
    }
}