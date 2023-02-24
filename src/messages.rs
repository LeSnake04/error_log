use crate::{Entry::Message, ErrorLog};
use log::LevelFilter;

#[macro_export]
/// Attach error message to given [ErrorLog]
macro_rules! log_error {
    ($errlog: ident, $($msg: tt)+) => {
        error_log::internal_log_push!($errlog, Error, $($msg)+);
    };
}
#[macro_export]
/// Attach warning to given [ErrorLog]
macro_rules! log_warn {
    ($errlog: ident, $($msg: tt)+) => {
        error_log::internal_log_push!($errlog, Warn, $($msg)+);
    };
}
#[macro_export]
/// Attach information to given [ErrorLog]
macro_rules! log_info {
    ($errlog: ident, $($msg: tt)+) => {
        error_log::internal_log_push!($errlog, Info, $($msg)+);
    };
}
#[macro_export]
/// Attach debug message to given [ErrorLog]
macro_rules! log_debug {
    ($errlog: ident, $($msg: tt)+) => {
        error_log::internal_log_push!($errlog, Debug, $($msg)+);
    };
}
#[macro_export]
/// Attach trace message to given [ErrorLog]
macro_rules! log_trace {
    ($errlog: ident, $($msg: tt)+) => {
        error_log::internal_log_push!($errlog, Trace, $($msg)+);
    };
}

#[macro_export]
#[doc(hidden)]
/// Intended for internal use in other macros only.
macro_rules! internal_log_push (
    ($errlog: ident, $level: ident, $($msg:tt)+) => (
        $errlog.push_message(error_log::LevelFilter::$level, format!($($msg)+))
    )
);

impl<T, E> ErrorLog<T, E> {
    /// Set max [LevelFilter] of displayed messages
    pub fn set_max_level(&mut self, level: LevelFilter) -> &mut Self {
        self.max_level = level;
        self
    }
    /// Set different max [LevelFilter] of displayed messages depending on if this is an debug build or a release one.
    pub fn set_max_level_debug(&mut self, release: LevelFilter, debug: LevelFilter) -> &mut Self {
        self.max_level = match cfg!(debug_assertions) {
            true => debug,
            false => release,
        };
        self
    }
    /// Get max [LevelFilter]
    pub fn max_level(&self) -> &LevelFilter {
        &self.max_level
    }
    /** Push Message to entries.
    /
    / Its recommended to use the built in macros instead:
    / TODO: Reference macros here
    */
    pub fn push_message(&mut self, level: LevelFilter, msg: impl Into<String>) -> &mut Self {
        self.entries.push(Message {
            level,
            message: msg.into(),
        });
        self
    }
}
