#![feature(test)]
#[macro_use]
extern crate lazy_static;
extern crate test;

use crate::error::ReplicationError;

pub mod client;
pub mod common;
pub mod error;
pub mod loggerop;
pub mod mysql;
pub mod packet;
pub mod replication;
pub mod utils;

fn main() -> Result<(), ReplicationError> {
    loggerop::init_log()?;
    println!("Hello, world!");
    Ok(())
}
