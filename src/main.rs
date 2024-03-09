mod login;
mod status;
mod packet;
mod util;
mod storage;
mod prelude;
mod encrypt;
mod constants;
mod world;

use std::{net::UdpSocket, time::Instant};

use prelude::*;
use tracing_subscriber::filter::LevelFilter;
use world::WorldHelper;

fn main() {
    init();
    run();
}

fn init() {
    init_tokio_runtime();
    #[cfg(debug_assertions)]
    {
        let level = LevelFilter::DEBUG;
        tracing_subscriber::fmt().with_max_level(level).init();
        std::env::set_var("RUST_BACKTRACE", "1");
    }
    debug!("Initialized");
}

fn run() {
    let mut world_helper = create_world_helper();
    debug!("Running...");
    loop {
        let curr_time = Instant::now();
        world_helper.execute();
        let elapsed = curr_time.elapsed();
        if elapsed < fps_to_duration(60) {
            std::thread::sleep(Duration::from_millis(1));
        }
    }
}

fn create_world_helper() -> WorldHelper {
    let mut world_helper = WorldHelper::new();

    storage::init(&mut world_helper);
    login::init(&mut world_helper);
    status::init(&mut world_helper);

    world_helper
}

/*
fn add_plugins(app: &mut App) {
    app
        .add_plugins(MinimalPlugins)
        .add_plugins(TokioTasksPlugin::default()) /* to use tokio runtime */
        .add_plugins(LogPlugin::default());
}
*/