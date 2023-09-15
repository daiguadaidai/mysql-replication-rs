use crate::error::ReplicationError;
use lazy_static::lazy_static;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use std::sync::Mutex;

lazy_static! {
    static ref IS_INIT_LOG: Mutex<bool> = Mutex::new(false);
}

pub fn init_log_once() -> Result<(), ReplicationError> {
    let mut is_init = IS_INIT_LOG.lock().unwrap();
    if !*is_init {
        init_log()?;
        *is_init = true;
    }

    return Ok(());
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
        ReplicationError::new(format!(
            "init log4rs config failed. err: {e}",
            e = e.to_string()
        ))
    })?;

    let _ = log4rs::init_config(config)?;

    Ok(())
}
