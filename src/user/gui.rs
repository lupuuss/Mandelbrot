use super::ModeRunner;
use super::Config;
use super::worker::Worker;

use super::super::fractal as fractal;
use fractal::Fractal;
use fractal::trans::{FramePart, SurfaceWriter};

use sdl2::event::Event;
use std::time::Duration;
pub struct GuiRunner {}

impl GuiRunner {
    pub fn new() -> Self {
        GuiRunner {}
    }
}

impl ModeRunner for GuiRunner {

    fn start(&mut self, config: &Config) {

        let mandelbrot = Fractal::new_thread_safe_generator(
            Fractal::Mandelbrot(config.pixel_range(), config.max_iterations())
        );

        let mut worker: Worker<FramePart> = Worker::new(config.threads(), false);
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
    
        let (width, height) = config.pixel_range();

        let window = video_subsystem.window("Mandelbrot", width as u32, height as u32)
            .position_centered()
            .build()
            .unwrap();
    

        let mut event_pump = sdl_context.event_pump().unwrap();

        Fractal::generate_frame_on_worker(
            mandelbrot, 
            config.complex_range(),
            config.thread_split() * config.threads(),
            &mut worker
        );

        let reciver = worker.output_receiver();

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..}  => {
                        break 'running;
                    },
                    _ => {}
                }
            }
            
            let result = reciver.try_recv();

            if let Ok(frame_part) = result {
                let mut surface_writer = SurfaceWriter::new(window.surface(&event_pump).unwrap());

                surface_writer.write_part(frame_part, config.max_iterations());
                surface_writer.update_window().unwrap();
            }
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}