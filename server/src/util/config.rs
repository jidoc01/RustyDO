use crate::prelude::*;

use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize};
use config;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub server: ConfigServer,
    pub user: ConfigUser,
    pub message: ConfigMessage,
}

#[derive(Clone, Deserialize)]
pub struct ConfigServer {
    pub password: String,
    pub use_auto_account: bool,
    pub max_users: usize,
}

#[derive(Clone, Deserialize)]
pub struct ConfigUser {
    pub initial_level: u8,
    pub initial_money: u32,
}

#[derive(Clone, Deserialize)]
pub struct ConfigMessage {
    pub notice: String,
}

fn get_random_string(len: usize) -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), len)
}

impl Config {
    pub fn open(path: &str) -> Result<Self> {
        let mut ret: Self = config::Config::builder()
            .add_source(config::File::with_name(path))
            .build()?
            .try_deserialize()?;
        let pw = &ret
            .server
            .password;
        if pw.is_empty() {
            ret
                .server
                .password = get_random_string(32);
        }
        Ok(ret)
    }
}