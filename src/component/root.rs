use std::{cell::RefCell, rc::Rc};

use crate::component::{Component, Updatable, world::World};
use color_eyre::Result;

pub struct Root {
    components: Vec<Rc<RefCell<dyn Component>>>,
    updatables: Vec<Rc<RefCell<dyn Updatable>>>,
}

impl Default for Root {
    fn default() -> Self {
        Self::new()
    }
}

impl Root {
    pub fn new() -> Self {
        let world = Rc::new(RefCell::new(World::new()));
        Self {
            components: vec![world.clone()],
            updatables: vec![world.clone()],
        }
    }
}

impl Component for Root {
    fn name(&self) -> &'static str {
        "Root"
    }

    fn draw(&self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) -> Result<()> {
        for component in &self.components {
            component.borrow_mut().draw(frame, area)?;
        }
        Ok(())
    }
}

impl Updatable for Root {
    fn update(&mut self) -> Result<()> {
        let updatables = &self.updatables;
        for updatable in updatables {
            updatable.borrow_mut().update()?;
        }
        Ok(())
    }
}
