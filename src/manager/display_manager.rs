use std::{error::Error, sync::Arc};

use winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use crate::world::Vector;

pub struct DisplayManager {
    event_loop: EventLoop<()>,
    window: Arc<Window>,
}

impl DisplayManager {
    pub fn get_width(&self) -> u32 {
        self.window.inner_size().width
    }

    pub fn get_height(&self) -> u32 {
        self.window.inner_size().height
    }

    pub fn draw_ch(
        &self,
        _world_pos: Vector,
        _ch: char,
        _color: Color,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

pub struct DisplayManagerBuilder {
    width: usize,
    height: usize,
    title: String,
}

impl DisplayManagerBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_size(&mut self, width: usize, height: usize) -> &mut Self {
        self.width = width;
        self.height = height;

        self
    }

    pub fn with_title(&mut self, title: String) -> &mut Self {
        self.title = title;

        self
    }

    pub fn build(&self) -> DisplayManager {
        let event_loop = EventLoop::new().unwrap();
        let window = Arc::new(
            WindowBuilder::new()
                .with_inner_size(LogicalSize::new(self.width as f64, self.height as f64))
                .with_title(self.title.clone())
                .build(&event_loop)
                .unwrap(),
        );

        DisplayManager { event_loop, window }
    }
}

impl Default for DisplayManagerBuilder {
    fn default() -> Self {
        Self {
            width: 1024,
            height: 768,
            title: String::from("Rustifly Engine"),
        }
    }
}

pub enum Color {
    UndefinedColor = -1,
    Black = 0,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}
