#![feature(test)]
#[macro_use]
extern crate lazy_static;
extern crate test;

use crate::error::ReplicationError;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;

pub mod error;
pub mod mysql;
pub mod replication;
pub mod utils;

fn main() -> Result<(), ReplicationError> {
    init_log()?;
    println!("Hello, world!");
    Ok(())
}

pub fn init_log() -> Result<(), ReplicationError> {
    let pattern = "{d(%Y-%m-%d %H:%M:%S)} - {l} - {f}::{L} - {m}{n}";
    // 控制台输出
    let console_appender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build();

    // 一定会打印控制台
    let root = Root::builder().appender("stdout").build(LevelFilter::Info);

    let config_builder = log4rs::Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(console_appender)));
    let config = config_builder.build(root).map_err(|e| {
        ReplicationError::new(format!("初始化日志配置出错. err: {e}", e = e.to_string()))
    })?;

    let _ = log4rs::init_config(config)?;

    Ok(())
}
