use std::convert::TryInto;

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
                let square = rectangle::square(0.0, 0.0, 2.);
                
                self.gl.as_mut().unwrap().draw(args.viewport(), |c, gl| {
                    clear([0., 0., 0., 1.], gl);
                    
                    for i in 0..240{
                        for j in 0..256{
                            let transform = c
                                .transform
                                .trans(j as f64 * 2., i as f64 * 2.);
                            let pixel = buf.read_pixel(i, j);
                            let color = [buf.read_pixel(i, j)];
                            rectangle([pixel.r as f32 / 255., pixel.g as f32 / 255., pixel.b as f32 / 255., 1.], square, transform, gl);
                        }
                    }
                });
            }
            return Ok(true);
        }
        return Ok(false);
    }
}