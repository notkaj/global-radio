use color_eyre::Result;

use crate::component::Component;

#[allow(dead_code)]
pub struct Country {
    pub name: String,
    pub code: String,
    pub station_count: u16,
}

impl Component for Country {
    fn name(&self) -> &'static str {
        "Country"
    }

    fn draw(&self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) -> Result<()> {
        Ok(())
    }
}
