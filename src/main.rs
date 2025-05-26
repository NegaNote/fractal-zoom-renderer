use winit::event_loop::EventLoop;
use anyhow::Result;
use winit::window::{Window, WindowAttributes};

fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;
    println!("Hello, world!");
    Ok(())
}
