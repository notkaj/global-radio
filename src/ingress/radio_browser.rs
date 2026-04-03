use std::sync::OnceLock;

use color_eyre::Result;
use radiobrowser::{self, ApiCountry, ApiStation, CountryOrder, RadioBrowserAPI};

static CONTEXT: OnceLock<Context> = OnceLock::new();
pub async fn init_context() -> Result<()> {
    let context = Context::build().await?;
    CONTEXT.get_or_init(|| context);
    Ok(())
}

pub fn context() -> &'static Context {
    let context = CONTEXT.get();
    match context {
        Some(c) => c,
        None => panic!("Context has not been initialized"),
    }
}

pub struct Context {
    api: RadioBrowserAPI,
}

impl Context {
    // pub const fn new() -> Self {
    //     Self { api: None }
    // }

    pub async fn build() -> Result<Self> {
        let api = RadioBrowserAPI::new().await?;
        Ok(Self { api })
    }

    #[allow(dead_code)]
    async fn stations_by_name(&self, name: String) -> Result<Vec<ApiStation>> {
        Ok(self
            .api
            .get_stations()
            .name(name)
            .order(radiobrowser::StationOrder::Clickcount)
            .send()
            .await?)
    }

    pub async fn countries(&self) -> Result<Vec<ApiCountry>> {
        Ok(self.api.get_countries().send().await?)
    }

    pub async fn countries_by_order(&self, order: CountryOrder) -> Result<Vec<ApiCountry>> {
        Ok(self.api.get_countries().order(order).send().await?)
    }
}
