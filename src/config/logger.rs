use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;
use crate::tools::capitalize::capitalize;

pub fn config(log_level: String) {
    let log_level = capitalize(&log_level).parse::<LevelFilter>().unwrap_or(LevelFilter::Info);

    Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                     "{} [{}] - {}",
                     Local::now().format("%Y-%m-%dT%H:%M:%S"),
                     record.level(),
                     record.args()
            )
        })
        .filter(None, log_level)
        .init();
}