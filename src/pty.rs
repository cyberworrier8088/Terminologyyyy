// src/pty.rs

// pty means pseudo terminal
// it is used for running shell commands
// it is a linux / unix thing
// in windows it is called conpty
// this run every where 
// FAQ: 
//     Q. What is pty? Google it! get A
//     A. a software-defined, virtual device pair in Unix-like OS that emulates a phyiscal hardware terminal    
// for more info: https://docs.rs/portable-pty/latest/portable_pty/
//     Q. Why we using that? A. Because we want to run shell commands in our terminal and more easyly use. (maybe feature i build my own)
// thankyou

use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};


pub struct Pty {
    pub pair: portable_pty::PtyPair,
}


impl Pty {
    pub fn new() -> Self {
        let pty_system = NativePtySystem::default();

        let pair = pty_system.openpty(PtySize {
            rows: 30,
            cols: 120,
            pixel_width: 0,
            pixel_height: 0,
        }   
        ).unwrap();

        #[cfg(target_os = "windows")]
        let cmd = CommandBuilder::new("powershell.exe");

        #[cfg(not(target_os = "windows"))]
        let cmd = CommandBuilder::new("bash");

        pair.slave.spawn_command(cmd).unwrap();

        Self {
            pair,
        }

    }
    
}