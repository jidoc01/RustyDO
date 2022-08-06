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