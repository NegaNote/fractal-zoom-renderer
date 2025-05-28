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
                            *(inner[0].1) = colored(
                                x,
                                y,
                                width.get(),
                                height.get(),
                                Fractal::Mandelbrot(2),
                                RenderBox {
                                    left: -2.5,
                                    right: 1.0,
                                    bottom: -1.0,
                                    top: 1.0,
                                },
                            );
                        });

                    buffer.present().unwrap();
                }
            }
            _ => (),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Fractal {
    Mandelbrot(i32),
    Julia(i32, Complex64),
}

#[derive(Debug, Copy, Clone)]
struct RenderBox {
    left: f64,
    right: f64,
    bottom: f64,
    top: f64,
}

fn colored(
    x: usize,
    y: usize,
    width: u32,
    height: u32,
    fractal: Fractal,
    render_box: RenderBox,
) -> u32 {
    let real_coord = (x as f64 / width as f64) * (render_box.right - render_box.left) + render_box.left;

    let imag_coord = (y as f64 / height as f64) * (render_box.bottom - render_box.top) + render_box.top;

    let max_iterations = 1000;
    
    let iters = match fractal {
        Fractal::Mandelbrot(power) => {
            let mut c = Complex64::new(real_coord, imag_coord);
            let mut z = Complex64::new(0.0, 0.0);
            iterations(&mut z, &mut c, max_iterations, power)
        },
        Fractal::Julia(power, mut c) => {
            let mut z = Complex64::new(real_coord, imag_coord);
            iterations(&mut z, &mut c, max_iterations, power)
        }
    };

    let red = ((iters as f64) * 255.0 / (max_iterations as f64)) as u32;
    let green = ((iters as f64) * 255.0 / (max_iterations as f64)) as u32;
    let blue = ((iters as f64) * 255.0 / (max_iterations as f64)) as u32;

    blue | (green << 8) | (red << 16)
}

fn iterations(z: &mut Complex64, c: &mut Complex64, max_iterations: i32, power: i32) -> i32 {
    let mut iters = 0;
    for _ in 0..max_iterations {
        iters += 1;
        *z = z.powi(power) + *c;
        if (z.re * z.re + z.im * z.im) > 4.0 {
            break;
        }
    }
    iters
}

fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();
    event_loop.run_app(&mut app)?;
    Ok(())
}
