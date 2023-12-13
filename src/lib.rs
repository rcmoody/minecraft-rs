//! A Minecraft clone written in Rust.

mod renderer;

use anyhow::{Context, Result};
use log::error;
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
    pub async fn new() -> Result<Self> {
        let event_loop = EventLoop::new().context("Failed to create the event loop")?;
        let window = WindowBuilder::new()
            .with_title("Minecraft")
            .build(&event_loop)
            .context("Failed to the create window")?;

        let renderer = Renderer::new(&window).await;

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
                    ref event,
                    window_id,
                } if window_id == self.window.id() => match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::Resized(physical_size) => {
                        self.resize(*physical_size);
                    }
                    _ => {}
                },
                Event::AboutToWait => {
                    self.update();

                    if let Err(e) = self.render() {
                        match e {
                            wgpu::SurfaceError::Lost => self.resize(self.size()),
                            wgpu::SurfaceError::OutOfMemory => elwt.exit(),
                            _ => error!("{e}"),
                        }
                    }
                }
                _ => (),
            })
            .context("Failed to start the event loop")?;

        Ok(())
    }

    fn size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.renderer.size()
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.renderer.resize(new_size);
    }

    fn update(&mut self) {
        // ...
    }

    fn render(&self) -> Result<(), wgpu::SurfaceError> {
        self.renderer.render()?;

        Ok(())
    }
}
