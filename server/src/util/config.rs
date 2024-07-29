// RustyDO
//
// Copyright 2022. JungHyun Kim (jidoc01).
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU Affero General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more
// details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

use crate::prelude::*;

use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize};
use ::config;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub server: ConfigServer,
    pub user: ConfigUser,
    pub message: ConfigMessage,
    pub ticker: ConfigTicker,
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

#[derive(Clone, Deserialize)]
pub struct ConfigTicker {
    pub initial: String,
    pub board: String,
    pub ranking: String,
    pub selection: String,
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