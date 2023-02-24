use pretty_assertions::assert_eq;

use std::num::ParseIntError;

use error_log::{
    log_debug, log_error, log_info, log_trace, log_warn, Entries, Entry, ErrorLog, LevelFilter,
};

#[test]
fn no_filter() {
    test_message(
        None,
        vec![
            Entry::Message {
                level: LevelFilter::Error,
                message: String::from("error"),
            },
            Entry::Message {
                level: LevelFilter::Warn,
                message: String::from("warn"),
            },
            Entry::Message {
                level: LevelFilter::Info,
                message: String::from("info"),
            },
            Entry::Message {
                level: LevelFilter::Debug,
                message: String::from("debug"),
            },
            Entry::Message {
                level: LevelFilter::Trace,
                message: String::from("trace"),
            },
        ],
    )
}
#[test]
fn messages_max_level() {
    test_message(
        Some(LevelFilter::Warn),
        vec![
            Entry::Message {
                level: LevelFilter::Error,
                message: String::from("error"),
            },
            Entry::Message {
                level: LevelFilter::Warn,
                message: String::from("warn"),
            },
        ],
    )
}

fn test_message(max_level: Option<LevelFilter>, expected: Entries<ParseIntError>) {
    let mut err_log = ErrorLog::<i32, ParseIntError>::new();
    if let Some(max) = max_level {
        err_log.set_max_level(max);
    }
    log_error!(err_log, "error");
    log_warn!(err_log, "warn");
    log_info!(err_log, "info");
    log_debug!(err_log, "debug");
    log_trace!(err_log, "trace");
    assert_eq!(err_log.entries_cloned(), expected);
    assert!(err_log.display_ok().is_none());
}
