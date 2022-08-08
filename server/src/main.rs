// Copyright 2022 JungHyun Kim
// This file is part of RustyDO.
// RustyDO is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// RustyDO is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License along with RustyDO. If not, see <https://www.gnu.org/licenses/>.

mod server;
mod util;
mod prelude;

extern crate anyhow;

// TODO: Do not import rusqlite twice.
use rusqlite::NO_PARAMS;

use crate::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
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