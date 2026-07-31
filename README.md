# Terminologyyyy

A simple terminal emulator written in Rust using 'wgpu', 'glyphon', 'winit', and 'portable-pty'



## Features

- Run PowerShell (Windows) or Bash (linux and mac :)
- PTY (Pseudo Terminal)
- Keyboard input
- ANSI escape sequence support
- CSI escape sequence support
- OSC parsing
- Cursor movement
- Screen buffer
- Automatic line wrapping
- Scrollback history (5000 lines)
- Mouse wheel scrolling
- Viewport scrolling
- Automatic resize (rows & columns)
- Text rendering with GPU
- ANSI text colors

## Built With

- Rust
- wgpu
- glyphon
- winit
- portable-pty

## Current Status

Implemented:

- Terminal rendering
- PTY communication
- ANSI parser
- Scrollback
- Mouse scrolling
- Dynamic resize

Planned:

- Cursor blinking
- Copy & Paste
- Text selection
- 256-color and True Color support
- Alternate screen buffer
- Mouse reporting
- Better performance

## Run

```bash
cargo run
```

## License

MIT