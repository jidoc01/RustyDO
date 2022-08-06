
/*
    Import frequently-used modules.
*/
pub use crate::{*, util::*, server::*};
pub use std::sync::{Arc}; // To prevent the confliction between std and tokio, we selectively use modules from std.
pub use std::cell::RefCell;
pub use tokio::sync::*;
pub use async_trait::*; // For async traits.
pub use anyhow::*;
pub use std::result::Result::{Ok, Err};

/*
    Constants.
*/
pub const DB_PATH: &str = "./db.nosqlite";
pub const CONFIG_PATH: &str = "./config.toml";

pub const HEADER_SIZE: usize = 9;
pub const TAIL_SIZE: usize = 3;

pub const MAX_USERS: usize = 1000;

pub const ITEM_COUNT: usize = 4;
pub const EXP_COUNT: usize = 8;
pub const MACRO_COUNT: usize = 8;

pub const MIN_CLIENT_UID: ClientId = 1;
pub const MAX_CLIENT_UID: ClientId = MAX_USERS as ClientId;

/*
    Types.
*/
pub type ClientId = u16;

// pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/*
    APIs.
*/
pub fn read_u16(vec: &[u8], off: usize) -> u16 {
    ((vec[off+0] as u16) << 0) | ((vec[off+1] as u16) << 8)
}

pub fn read_u32(vec: &[u8], off: usize) -> u32 {
    ((vec[off+0] as u32) << 0)
    | ((vec[off+1] as u32) << 8)
    | ((vec[off+2] as u32) << 16)
    | ((vec[off+3] as u32) << 24)
}

pub fn concat_list_of_vec(list: &[&Vec<u8>]) -> Vec<u8> {
    let mut out = vec!();
    for bytes in list {
        out.extend_from_slice(bytes);
    }
    out
}

#[macro_export]
macro_rules! run {
    ($x: expr) => {
        {
            tokio::spawn($x);
        }
    };
}

#[inline]
pub fn if_else<T>(cond: bool, t: T, f: T) -> T {
    if cond {
        t
    } else {
        f
    }
}

#[allow(unused_macros)]

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! log {
    ($( $args:expr ),*) => { println!( $( $args ),* ); }
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! log {
    ($( $args:expr ),*) => {()}
}

