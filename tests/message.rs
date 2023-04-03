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
            Entry::new_message(LevelFilter::Error, String::from("error")),
            Entry::new_message(LevelFilter::Warn, String::from("warn")),
            Entry::new_message(LevelFilter::Info, String::from("info")),
            Entry::new_message(LevelFilter::Debug, String::from("debug")),
            Entry::new_message(LevelFilter::Trace, String::from("trace")),
        ],
    )
}
#[test]
fn messages_max_level() {
    test_message(
        Some(LevelFilter::Warn),
        vec![
            Entry::new_message(LevelFilter::Error, String::from("error")),
            Entry::new_message(LevelFilter::Warn, String::from("warn")),
        ],
    )
}

fn test_message(max_level: Option<LevelFilter>, expected: Entries<ParseIntError>) {
    let mut err_log = ErrorLog::<i32, ParseIntError>::new();
    if let Some(max) = max_level {
        err_log.max_level(max);
    }
    log_error!(err_log, "error");
    log_warn!(err_log, "warn");
    log_info!(err_log, "info");
    log_debug!(err_log, "debug");
    log_trace!(err_log, "trace");
    assert_eq!(
        expected.clear_timestamps(),
        err_log.entries_cloned().clear_timestamps()
    );
    assert!(err_log.display_ok().is_none());
}
