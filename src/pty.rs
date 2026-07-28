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

use std::io::Read;
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::io::Write;

pub struct Pty {
    pub pair: portable_pty::PtyPair,
    rx: Receiver<String>,
    writer: Box<dyn Write + Send>,
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

        let writer = pair.master.take_writer().unwrap();

        #[cfg(target_os = "windows")]
        let cmd = CommandBuilder::new("powershell.exe");

        #[cfg(not(target_os = "windows"))]
        let cmd = CommandBuilder::new("bash");

        pair.slave.spawn_command(cmd).unwrap();

        let mut reader = pair.master.try_clone_reader().unwrap();
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let mut buffer = [0u8; 4096];
            while let Ok(n) = reader.read(&mut buffer) {
                if n == 0 {
                    break;
                }
                let s = String::from_utf8_lossy(&buffer[..n]).to_string();
                if tx.send(s).is_err() {
                    break;
                }
            }
        });

        Self {
            pair,
            rx,
            writer
        }

    }

    pub fn read_output(&mut self) -> String {
        let mut output = String::new();
        while let Ok(chunk) = self.rx.try_recv() {
            output.push_str(&chunk);
        }
        output
    }

    pub fn write(&mut self, text: &str) {
        use std::io::Write;

        self.writer.write_all(text.as_bytes()).unwrap();

        self.writer.flush().unwrap();
    }
    
}