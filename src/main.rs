mod login;
mod status;
mod packet;
mod util;
mod storage;
mod prelude;
mod encrypt;
mod constants;
mod world;

use prelude::*;
use world::WorldHelper;

fn main() {
    /*
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).unwrap();
    */
    init_tokio_runtime();
    tracing_subscriber::fmt().init();
    let mut world_helper = create_world_helper();
    loop {
        world_helper.execute();
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