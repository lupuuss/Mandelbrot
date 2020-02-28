use super::ModeRunner;
use super::Config;
use super::BaseRunner;
use super::worker::Worker;

use super::super::fractal as fractal;
use fractal::Fractal;
use fractal::trans::{FramePart, SurfaceWriter};

use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::mouse::MouseWheelDirection;

use std::time::Duration;

pub struct GuiRunner {
    base: BaseRunner
}

impl GuiRunner {
    pub fn new(config: Config, generator: Fractal) -> Self {
        GuiRunner {
            base: BaseRunner::new(config, generator)
        }
    }
}

impl ModeRunner for GuiRunner {

    fn start(&mut self) {

        let config = self.base.config();

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
    
        let (width, height) = config.pixel_range();

        let window = video_subsystem.window("Mandelbrot", width as u32, height as u32)
            .position_centered()
            .build()
            .unwrap();
    

        let mut event_pump = sdl_context.event_pump().unwrap();
        let full_split = config.thread_split() * config.threads();
        let mut range =config.complex_range();

        let mut worker: Worker<FramePart> = Worker::new(config.threads(), true);

        let generator = self.base.generator();

        let mut events_handler = EventsHandler::new(
            generator.read().unwrap().between_pixels(range),
            20.0,
            config.pixel_range()
        );

        let mut changes_occured = true;

        loop {
            
            events_handler.handle(&mut event_pump);

            if events_handler.quit() {
                break;
            } 

            if let Some(mv) = events_handler.range_move() {

                range.move_range(mv);
                changes_occured = true;

            }
            
            if let Some(shrink) = events_handler.range_shrink() {

                range.shrink_range(shrink);
                events_handler.update_particles(self.base.generator().read().unwrap().between_pixels(range));
                changes_occured = true;
            }
            
            if changes_occured && !worker.is_occupied() {

                Fractal::generate_frame_on_worker(
                    self.base.generator(), 
                    range,
                    full_split,
                    &mut worker
                );
                changes_occured = false;
            }

            for frame_part in worker.output_receiver().try_iter() {

                let mut surface_writer = SurfaceWriter::new(window.surface(&event_pump).unwrap());

                surface_writer.write_part(frame_part, config.max_iterations());
                surface_writer.update_window().unwrap();
            }

            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 300));
        }
    }
}


struct EventsHandler {
    mouse_move: Option::<(i32, i32)>,
    wheel_move_y: Option::<i32>,
    particles: (f64, f64),
    screen_prop: f64,
    calculated_range_move: Option<(f64, f64)>,
    shrink_modifier: f64,
    quit: bool
}

impl EventsHandler {

    fn new(particles: (f64, f64), shrink_modifier: f64, size: (usize, usize)) -> Self {
        EventsHandler {
            mouse_move: None,
            wheel_move_y: None,
            particles: particles,
            screen_prop: size.0 as f64 / size.1 as f64,
            calculated_range_move: None,
            shrink_modifier: shrink_modifier,
            quit: false
        }
    }

    fn handle(&mut self, event_pump: &mut EventPump) {

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}  => {
                    self.quit = true;
                },

                Event::MouseMotion { mousestate, xrel, yrel, .. } => {
                    
                    if mousestate.left() && (xrel != 0 || yrel != 0) {
                        let tmp = self.mouse_move.get_or_insert((0, 0));
                        self.mouse_move = Some((tmp.0 + xrel, tmp.1 + yrel));
                    }
                }

                Event::MouseWheel { y, direction, ..} if direction == MouseWheelDirection::Normal => {

                    *self.wheel_move_y.get_or_insert(0) += y;
                }
                _ => {}
            }
        }
    }

    fn range_move(&mut self) -> Option<(f64, f64)> {
        
        if let Some(mv) = self.mouse_move.take() {

            let range_mv = (
                -self.particles.0 * mv.0 as f64,
                self.particles.1 * mv.1 as f64
            );                    

            self.calculated_range_move = Some(range_mv);
        }

        return self.calculated_range_move.take()
    }

    fn range_shrink(&mut self) -> Option<(f64, f64)> {
        
        let res_move_y = self.wheel_move_y.take();

        if let Some(move_y) = res_move_y {

            let modifier = self.shrink_modifier * move_y as f64;

            return Some(
                (self.particles.0 * modifier * self.screen_prop , self.particles.1 * modifier )
            );
        }

        return None;
    }

    fn quit(&self) -> bool {
        self.quit
    }

    fn update_particles(&mut self, particles: (f64, f64)) {
        self.particles = particles;
    }
}