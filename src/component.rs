use color_eyre::Result;
use ratatui::{Frame, layout::Rect};

pub mod country;
pub mod root;
pub mod world;

pub trait Component {
    fn name(&self) -> &'static str;
    fn draw(&self, frame: &mut Frame, area: Rect) -> Result<()>;
}

pub trait Populatable: Component {
    fn populate(&mut self) -> Result<()>;
}

pub trait Updatable: Component {
    fn update(&mut self) -> Result<()>;
}
