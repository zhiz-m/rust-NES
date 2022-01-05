use crate::displays::display::{ScreenBuffer};

pub trait Frontend {
    fn new() -> Self;
    
    // may return an error. 
    fn start(&mut self) -> Result<(), &'static str>;
    
    // currently, the only data passed to the frontend are the pixel details of the NES. 
    // it is expected that more data may need to be piped between the frontend and backend in future. 
    // returns false if frontend has exited, otherwise true. May return an error. 
    fn render(&mut self, buf: &ScreenBuffer) -> Result<bool, &'static str>;
}