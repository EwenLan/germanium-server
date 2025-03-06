use chrono::Local;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Config;

static LOG_PATTERN: &str = "[{h([{l}])}[{d(%+)}][{f}:{L}][{m}]]{n}";

pub(crate) fn configure_logging() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all("log")?;

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(LOG_PATTERN)))
        .build();

    let current_date = Local::now().format("%Y-%m-%d").to_string();
    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(LOG_PATTERN)))
        .build(format!("log/{}.log", current_date))?;

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file", Box::new(file)))
        .build(
            Root::builder()
                .appender("stdout")
                .appender("file")
                .build(LevelFilter::Debug),
        )?;

    log4rs::init_config(config)?;
    Ok(())
}
