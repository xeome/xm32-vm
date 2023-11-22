use env_logger::fmt::Color;
use log::Level;
use std::{env, io::Write};

pub fn init_logger() {
    // If RUST_LOG is not set then set it
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }
    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            let level = record.level();
            let mut style = buf.style();
            match record.level() {
                Level::Error => style.set_color(Color::Red),
                Level::Warn => style.set_color(Color::Yellow),
                Level::Info => style.set_color(Color::Green),
                Level::Debug => style.set_color(Color::Blue),
                Level::Trace => style.set_color(Color::Cyan),
            };

            writeln!(
                buf,
                "{}:{} {} [{}] - {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                chrono::Local::now().format("%H:%M:%S"),
                style.value(level),
                record.args()
            )
        })
        .init();
}
