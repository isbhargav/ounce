use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size};
use std::io::stdout;

/*
Terminal is responsilbe for following things
- reading the input form stdout
- dispatching proper events based on inputs
- Entering/Exiting Raw mode
*/

pub struct Terminal {
    width: u16,
    height: u16,
}

impl Terminal {
    pub fn new() -> Self {
        let (w, h) = size().unwrap();
        return Self {
            width: w,
            height: h,
        };
    }
    pub fn enable_raw() -> Result<(), std::io::Error> {
        return enable_raw_mode();
    }
    pub fn disable_raw() -> Result<(), std::io::Error> {
        return disable_raw_mode();
    }
    /*
    read key from termianl
    */
    pub fn read_key(&self) -> Result<crossterm::event::Event, std::io::Error> {
        let ev = crossterm::event::read()?;
        /* crossterm event -> editor_event -> action */
        return Ok(ev);
    }
}
