use std::{num::NonZeroU8, panic, thread};

use actix_web::middleware;
use time::{
    format_description::well_known::{
        self,
        iso8601::{Config, EncodedConfig, TimePrecision},
    },
    UtcOffset,
};
use tracing::{error, level_filters::LevelFilter, warn};
use tracing_subscriber::fmt::time::OffsetTime;

const MAX_LEVEL: LevelFilter = LevelFilter::INFO;
const CONFIG: EncodedConfig = Config::DEFAULT
    .set_time_precision(TimePrecision::Second {
        decimal_digits: NonZeroU8::new(3),
    })
    .encode();

pub fn init(writer: tracing_appender::non_blocking::NonBlocking) {
    let offset = match UtcOffset::current_local_offset() {
        Ok(o) => o,
        Err(e) => {
            warn!("can not get local offset, use UTC instead, {}", e);
            UtcOffset::UTC
        }
    };

    tracing_subscriber::fmt()
        .with_writer(writer)
        .with_timer(OffsetTime::new(offset, well_known::Iso8601::<CONFIG>))
        .with_max_level(MAX_LEVEL)
        .with_ansi(false)
        .init();

    log_panic();
}

pub fn custom_http_logger() -> middleware::Logger {
    middleware::Logger::new(r#"%a %r %s %Dms %b "%{Referer}i" "%{User-Agent}i" "%{ERROR}xo""#)
        .log_target("http")
        .custom_response_replace("ERROR", |res| match res.response().error() {
            Some(e) => e.to_string(),
            None => "-".to_string(),
        })
}

fn log_panic() {
    // catch panic and log them using tracing instead of default output to StdErr
    panic::set_hook(Box::new(|info| {
        let thread = thread::current();
        let thread = thread.name().unwrap_or("unknown");

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &**s,
                None => "Box<Any>",
            },
        };

        let backtrace = backtrace::Backtrace::new();

        match info.location() {
            Some(location) => {
                // without backtrace
                if msg.starts_with("notrace - ") {
                    error!(
                        target: "panic", "thread '{}' panicked at '{}': {}:{}",
                        thread,
                        msg.replace("notrace - ", ""),
                        location.file(),
                        location.line()
                    );
                }
                // with backtrace
                else {
                    error!(
                        target: "panic", "thread '{}' panicked at '{}': {}:{}\n{:?}",
                        thread,
                        msg,
                        location.file(),
                        location.line(),
                        backtrace
                    );
                }
            }
            None => {
                // without backtrace
                if msg.starts_with("notrace - ") {
                    error!(
                        target: "panic", "thread '{}' panicked at '{}'",
                        thread,
                        msg.replace("notrace - ", ""),
                    );
                }
                // with backtrace
                else {
                    error!(
                        target: "panic", "thread '{}' panicked at '{}'\n{:?}",
                        thread,
                        msg,
                        backtrace
                    );
                }
            }
        }
    }));
}
