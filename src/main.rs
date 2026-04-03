use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use futures::{FutureExt, StreamExt};
use ratatui::{DefaultTerminal, Frame};

use crate::{
    component::{Component, root::Root},
    ingress::radio_browser::init_context,
};

pub mod component;
pub mod ingress;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let mut app = App::new();
    app.init().await?;
    let result = App::new().run(terminal).await;
    ratatui::restore();
    result
}

#[derive(Debug, Default)]
pub struct App<T: Component> {
    /// Is the application running?
    running: bool,
    /// Event stream.
    event_stream: EventStream,
    /// Root component
    root_component: T,
}

impl App<Root> {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn init(&mut self) -> color_eyre::Result<()> {
        init_context().await
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_crossterm_events().await?;
        }
        Ok(())
    }

    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/master/examples>
    fn draw(&self, frame: &mut Frame) {
        if self.root_component.draw(frame, frame.area()).is_err() {
            panic!("App: failed to draw()")
        }
        // let title = Line::from("Ratatui Simple Template")
        //     .bold()
        //     .blue()
        //     .centered();
        // let text = "Hello, Ratatui!\n\n\
        //     Created using https://github.com/ratatui/templates\n\
        //     Press `Esc`, `Ctrl-C` or `q` to stop running.";
        // frame.render_widget(
        //     Paragraph::new(text)
        //         .block(Block::bordered().title(title))
        //         .centered(),
        //     frame.area(),
        // )
    }

    /// Reads the crossterm events and updates the state of [`App`].
    async fn handle_crossterm_events(&mut self) -> color_eyre::Result<()> {
        let event = self.event_stream.next().fuse().await;
        if let Some(Ok(e)) = event {
            match e {
                Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
                Event::Mouse(_) => {}
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            // Add other key handlers here.
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}
