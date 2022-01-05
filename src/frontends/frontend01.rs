use glutin_window::GlutinWindow as Window;
use graphics::{clear, Transformed, rectangle};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

use crate::displays::display::{ScreenBuffer,Pixel};
use super::frontend::Frontend;

pub struct Frontend01{
    gl: Option<GlGraphics>,
    window: Option<Window>,
    events: Option<Events>,
}

impl Frontend for Frontend01{
    fn new() -> Frontend01{
        return Frontend01 { 
            gl: None,
            window: None,
            events: None,
        }
    }
    
    fn start(&mut self) -> Result<(), &'static str>{
        self.window = Some(WindowSettings::new("rust-NES-frontend01", [480, 512])
            .graphics_api(OpenGL::V3_2)
            .exit_on_esc(true)
            .build()
            .unwrap());
        self.gl = Some(GlGraphics::new(OpenGL::V3_2));
        self.events = Some(Events::new(EventSettings::new()));
        return Ok(())
    }
    
    fn render(&mut self, buf: &ScreenBuffer) -> Result<bool, &'static str>{
        if let Some(e) = self.events.as_mut().unwrap().next(self.window.as_mut().unwrap()){
            if let Some(args) = e.render_args(){
                const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
                const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
                let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
                let square = rectangle::square(0.0, 0.0, 50.0);
                self.gl.as_mut().unwrap().draw(args.viewport(), |c, gl| {
                    // Clear the screen.
                    clear(GREEN, gl);
                    println!("called2");
                    let transform = c
                        .transform
                        .trans(x, y)
                        .rot_rad(2.)
                        .trans(-25.0, -25.0);
                    
                    // Draw a box rotating around the middle of the screen.
                    rectangle(RED, square, transform, gl);
                });
            }
            return Ok(true);
        }
        return Ok(false);
    }
}