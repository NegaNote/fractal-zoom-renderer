use anyhow::Result;
use rayon::prelude::*;
use softbuffer::{Context, Surface};
use std::num::NonZeroU32;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

#[derive(Default, Debug)]
struct App {
    window: Option<Arc<Window>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .expect("Failed to create window"),
        ));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let context = Context::new(self.window.as_ref().unwrap().as_ref())
                    .expect("Failed to create context");

                let window = self.window.as_ref().unwrap().as_ref();
                let mut surface = Surface::new(&context, window).expect("Failed to create surface");
                let size = window.inner_size();
                if let (Some(width), Some(height)) =
                    (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
                {
                    surface
                        .resize(width, height)
                        .expect("Failed to resize surface");
                    let mut buffer = surface.buffer_mut().expect("Failed to get buffer");
                    buffer
                        .as_mut()
                        .iter_mut()
                        .enumerate()
                        .collect::<Vec<(usize, &mut u32)>>()
                        .par_chunks_exact_mut(1)
                        .for_each(|inner| {
                            let index = inner[0].0;
                            let y = index / width.get() as usize;
                            let x = index % width.get() as usize;
                            *(inner[0].1) = (x + y) as u32;
                        });

                    buffer.present().unwrap();
                }
            }
            _ => (),
        }
    }
}

fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();
    event_loop.run_app(&mut app)?;
    Ok(())
}
