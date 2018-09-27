#![cfg_attr(feature = "cargo-clippy", allow(cast_lossless))]
#[cfg(not(target_arch = "wasm32"))]
extern crate gl;
#[cfg(not(target_arch = "wasm32"))]
extern crate glutin;
#[macro_use]
extern crate lazy_static;
extern crate nalgebra as na;
extern crate noise;
extern crate rand;
#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate serde;

extern crate gl_context;

extern crate resources;

extern crate unicode_normalization;

extern crate image;
extern crate rusttype;

pub mod camera;
pub mod graphics;
pub mod gui;
pub mod particles;
pub mod window;

use gl_context::{AbstractContext, Context};
use graphics::Drawable;
use particles::ParticleEngine;

use std::f32;

use camera::Camera;
use gui::*;
use window::*;

/// Holds application resources.
pub struct App {
    camera: camera::ArcBall,
    window: Window,
    time: f32,
    gui: Gui,
    state: State,
    particles: ParticleEngine,
}

/// Holds application state.
pub struct State {
    mouse_x: f64,
    mouse_y: f64,
    is_running: bool,
    highpass_filter: f32,
    lowpass_filter: f32,
    speed_multiplier: f32,
    transparency: f32,
}

impl State {
    /// Creates a new State instance with sane defaults.
    pub fn new() -> Self {
        State {
            mouse_x: 0.0,
            mouse_y: 0.0,
            is_running: true,
            highpass_filter: 0.0,
            lowpass_filter: 1.0,
            speed_multiplier: 0.5,
            transparency: 0.5,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    /// Starts the application.
    pub fn new() -> App {
        #[cfg(target_arch = "wasm32")] {
            stdweb::initialize();
        }
        App {
            window: Window::new("Brainstorm!", 900, 900),
            camera: camera::ArcBall::new(),
            time: 0.0,
            gui: Gui::new((1000.0, 1000.0)),
            state: State::new(),
            particles: ParticleEngine::new(),
        }
    }

    /// Runs the application for one frame.
    pub fn run(&mut self) -> bool {
        let context = Context::get_context();

        // Handle events
        for event in &self.window.get_events() {
            match event {
                Event::Resized(w, h) => self.window.set_size(*w as u32, *h as u32),
                _ => {}
            };
            self.gui
                .handle_event(&event, &mut self.state, self.window.get_size());
            self.camera.handle_events(&event);
        }

        // Update camera and particle system
        self.camera.update();
        self.particles.update(&self.state, &mut self.camera);

        // Clear screen
        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(Context::COLOR_BUFFER_BIT);

        // Draw everything
        let projection_matrix = self.camera.get_projection_matrix();
        self.particles.draw(&projection_matrix, &self.state);
        self.gui.draw();

        self.window.swap_buffers();
        self.time += 0.01;

        self.state.is_running
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
