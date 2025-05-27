use anyhow::Result;
use num_complex::Complex64;
use rayon::prelude::*;
use softbuffer::{Context, Surface};
use std::num::NonZeroU32;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::{PhysicalSize, Size};
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

#[derive(Default, Debug)]
struct App {
    window: Option<Arc<Window>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut attrs = Window::default_attributes();
        attrs.inner_size = Some(Size::Physical(PhysicalSize::new(700, 400)));
        self.window = Some(Arc::new(
            event_loop
                .create_window(attrs)
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
                            *(inner[0].1) = colored(x, y, width.get(), height.get());
                        });

                    buffer.present().unwrap();
                }
            }
            _ => (),
        }
    }
}

fn colored(x: usize, y: usize, width: u32, height: u32) -> u32 {
    let real_coord = (x as f64 / width as f64) * 3.5 - 2.5;

    let imag_coord = (y as f64 / height as f64) * -2.0 + 1.0;
    let coord = Complex64::new(real_coord, imag_coord);

    let max_iterations = 1000;

    let mut iterations = 0;
    let mut z = Complex64::new(0.0, 0.0);

    for _ in 0..max_iterations {
        iterations += 1;
        z = z * z + coord;
        if (z.re * z.re + z.im * z.im) > 4.0 {
            break;
        }
    }

    let red = ((iterations as f64) * 255.0 / (max_iterations as f64)) as u32;
    let green = ((iterations as f64) * 255.0 / (max_iterations as f64)) as u32;
    let blue = ((iterations as f64) * 255.0 / (max_iterations as f64)) as u32;

    blue | (green << 8) | (red << 16)
}

fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();
    event_loop.run_app(&mut app)?;
    Ok(())
}
