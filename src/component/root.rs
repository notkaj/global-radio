use crate::component::{
    Component,
    world::{self, World},
};
use color_eyre::Result;

pub struct Root {
    components: Vec<Box<dyn Component>>,
}

impl Default for Root {
    fn default() -> Self {
        Self::new()
    }
}

impl Root {
    pub fn new() -> Self {
        let world = Box::new(World::new());
        Self {
            components: vec![world],
        }
    }
}

impl Component for Root {
    fn name(&self) -> &'static str {
        "Root"
    }

    fn draw(&self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) -> Result<()> {
        for component in &self.components {
            component.draw(frame, area);
        }
        Ok(())
    }
}
