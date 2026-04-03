use color_eyre::Result;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::List;
use throbber_widgets_tui::Throbber;
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::component::country::Country;
use crate::component::{Component, Populatable, Updatable};
use crate::ingress::radio_browser::context;
use radiobrowser::ApiCountry;

pub struct World {
    countries: Option<Vec<Country>>,
    tx: Sender<Result<Vec<Country>>>,
    rx: Receiver<Result<Vec<Country>>>,
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

impl World {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(1);
        Self {
            countries: None,
            tx,
            rx,
        }
    }

    async fn retreive(tx: Sender<Result<Vec<Country>>>) {
        let _ = match context().countries().await {
            Ok(c) => {
                let countries = c.into_iter().map(|c| c.into()).collect();
                tx.send(Ok(countries)).await
            }
            Err(e) => tx.send(Err(e)).await,
        };
    }
}

impl Populatable for World {
    fn populate(&mut self) -> Result<()> {
        self.countries = None;
        let tx = self.tx.clone();
        tokio::spawn(Self::retreive(tx));
        Ok(())
    }
}

impl Component for World {
    fn name(&self) -> &'static str {
        "World"
    }

    fn draw(&self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        match &self.countries {
            Some(c) => {
                let items = c.iter().map(|c| c.name.to_owned());
                let list = List::new(items);
                frame.render_widget(list, area);
            }
            None => {
                let throbber = Throbber::default().label("Loading...");
                frame.render_widget(throbber, area);
            }
        };
        Ok(())
    }
}

impl From<ApiCountry> for Country {
    fn from(value: ApiCountry) -> Self {
        Self {
            name: value.name,
            code: value.iso_3166_1,
            station_count: value.stationcount as u16,
        }
    }
}

impl Updatable for World {
    fn update(&mut self) -> Result<()> {
        if self.countries.is_some() {
            return Ok(());
        }
        if let Ok(r) = self.rx.try_recv() {
            match r {
                Ok(c) => {
                    self.countries = Some(c);
                }
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}
