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

mod server;
mod util;
mod prelude;

extern crate anyhow;

use crate::prelude::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
const LOGO: &str = r"
    _____           _         _____   ____
    |  __ \         | |       |  __ \ / __ \
    | |__) |   _ ___| |_ _   _| |  | | |  | |
    |  _  / | | / __| __| | | | |  | | |  | |
    | | \ \ |_| \__ \ |_| |_| | |__| | |__| |
    |_|  \_\__,_|___/\__|\__, |_____/ \____/
                          __/ |
                         |___/               ";

#[tokio::main]
async fn main() -> Result<()> {
    println!("{LOGO}");
    println!("RustyDO v{VERSION}");
    println!("Repository: {REPOSITORY}");
    println!("Contact: {AUTHORS}");
    let config = Config::open(CONFIG_PATH)?;
    let db = Connection::open(DB_PATH)?;
    db.table(USER_TBL)?;
    db.table(POST_TBL)?;
    let mut login = login::Server::new(config.clone(), db);
    let login_tx = login.get_server_tx();
    status::run(config, login_tx).await;
    login.run().await; // Note that it will block the thread.
    Ok(())
}
