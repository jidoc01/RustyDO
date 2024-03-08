pub use std::collections::HashMap;
use std::future::Future;
pub use polodb_core::bson::*;
use tokio::runtime::Runtime;
//pub use bevy::prelude::*;
pub use std::sync::Arc;
pub use lazy_static::lazy_static;
pub use std::net::SocketAddr;
pub use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
pub use ignore_result::Ignore;
pub use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream, select};
pub use std::time::Duration;
pub use std::collections::HashSet;
pub use evenio::prelude::*;
pub use log::*;

pub use crate::util::writer::Writer;
pub use crate::util::reader::Reader;
pub use crate::constants::*;

pub type Without<T> = Not<With<T>>;

pub const fn fps_to_duration(fps: u32) -> Duration {
    let millis = 1000 / fps; /* TODO: maybe inaccurate */
    Duration::from_millis(millis as u64)
}

pub fn block_on<F: Future>(f: F) ->F::Output{
    rt().block_on(f)
    /*
    block_in_place(|| {
        tokio::runtime::Handle::current().block_on(f)
    })
    */
}

static mut RT: Option<Runtime> = None;

fn set_rt(rt: Runtime) {
    assert!(unsafe { RT.is_none() });
    unsafe {
        RT = Some(rt);
    }
}

fn rt() -> &'static Runtime {
    assert!(unsafe { RT.is_some() });
    unsafe {
        RT.as_ref().unwrap()
    }
}

pub fn init_tokio_runtime() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    set_rt(rt);
}