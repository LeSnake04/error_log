use crate::{Entry, ErrorLog};
use alloc::string::String;
use log::LevelFilter;

#[macro_export]
/// Attach error message to given [`ErrorLog`]
macro_rules! log_error {
    ($errlog: ident, $($msg: tt)+) => {
        $errlog.push_message(error_log::LevelFilter::Error, format!($($msg)+));
    };
}
#[macro_export]
/// Attach warning to given [`ErrorLog`]
macro_rules! log_warn {
    ($errlog: ident, $($msg: tt)+) => {
        $errlog.push_message(error_log::LevelFilter::Warn, format!($($msg)+));
    };
}
#[macro_export]
/// Attach information to given [`ErrorLog`]
macro_rules! log_info {
    ($errlog: ident, $($msg: tt)+) => {
        $errlog.push_message(error_log::LevelFilter::Info, format!($($msg)+));
    };
}
#[macro_export]
/// Attaches debug message to given [`ErrorLog`]
macro_rules! log_debug {
    ($errlog: ident, $($msg: tt)+) => {
        $errlog.push_message(error_log::LevelFilter::Debug, format!($($msg)+));
    };
}
#[macro_export]
/// Attach trace message to given [`ErrorLog`]
macro_rules! log_trace {
    ($errlog: ident, $($msg: tt)+) => {
        $errlog.push_message(error_log::LevelFilter::Trace, format!($($msg)+));
    };
}

impl<T, E> ErrorLog<T, E> {
    /// Sets max [`LevelFilter`] of displayed messages
    /// Note: when [`LevelFilter::Off`], all messages get ignored, but errors still show
    pub fn max_level(&mut self, level: LevelFilter) -> &mut Self {
        self.max_level = level;
        self
    }
    /// Sets different max [`LevelFilter`] of displayed messages depending on if this is an debug build or a release one.
    pub fn max_level_debug(&mut self, release: LevelFilter, debug: LevelFilter) -> &mut Self {
        self.max_level = match cfg!(debug_assertions) {
            true => debug,
            false => release,
        };
        self
    }
    /// Gets max [`LevelFilter`]. Any Message of Lower Priority will be igored
    pub fn get_max_level(&self) -> &LevelFilter {
        &self.max_level
    }
    /**
    Pushes Message to entries.
    Its recommended to use the built in macros instead:
    - [`log_error`]
    - [`log_warn`]
    - [`log_info`]
    - [`log_debug`]
    - [`log_trace`]
    */
    pub fn push_message(&mut self, level: LevelFilter, msg: impl Into<String>) -> &mut Self {
        self.entries.push(Entry::new_message(level, msg.into()));
        self
    }
}
