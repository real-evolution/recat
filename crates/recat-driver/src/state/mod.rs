mod data_context;

use recat_app::state::{AppConfig, AppState, DataContext};

use crate::config::RecatArgs;

use self::data_context::RecatDataContext;

pub(crate) type RecatAppState = AppStateImpl<RecatArgs, RecatDataContext>;

pub(crate) struct AppStateImpl<Config, Data> {
    config: Config,
    data: Data,
}

impl<Config, Data> AppState for AppStateImpl<Config, Data>
where
    Config: AppConfig,
    Data: DataContext,
{
    type Config = Config;
    type Data = Data;

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn data(&self) -> &Self::Data {
        &self.data
    }
}

impl RecatAppState {
    pub(crate) fn new(config: RecatArgs) -> anyhow::Result<Self> {
        Ok(Self {
            data: RecatDataContext::new(&config)?,
            config,
        })
    }
}
