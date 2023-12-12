//! A Minecraft clone written in Rust.

mod renderer;

use anyhow::{Context, Result};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use crate::renderer::Renderer;

pub struct Application {
    renderer: Renderer,
    window: Window,
    event_loop: Option<EventLoop<()>>,
}

impl Application {
    pub fn new() -> Result<Self> {
        let event_loop = EventLoop::new().context("Failed to create the event loop")?;
        let window = WindowBuilder::new()
            .with_title("Minecraft")
            .build(&event_loop)
            .context("Failed to the create window")?;

        let renderer = Renderer::new(&window);

        Ok(Self {
            renderer,
            window,
            event_loop: Some(event_loop),
        })
    }

    pub fn run(mut self) -> Result<()> {
        self.event_loop
            .take()
            .unwrap()
            .run(move |event, elwt| match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    elwt.exit();
                }
                Event::AboutToWait => {
                    self.update();
                    self.draw();
                }
                _ => (),
            })
            .context("Failed to start the event loop")?;

        Ok(())
    }

    fn update(&mut self) {
        // ...
    }

    fn draw(&self) {
        self.renderer.draw();
    }
}
