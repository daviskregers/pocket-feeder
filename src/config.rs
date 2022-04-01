use serde::{Deserialize, Serialize};
use crate::source::Source;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub pocket: PocketConfiguration,
    pub sentry: Option<SentryConfiguration>,
    pub sources: Vec<Source>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PocketConfiguration {
    pub consumer: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SentryConfiguration {
    pub dsn: Option<String>,
}
