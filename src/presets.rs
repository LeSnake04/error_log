use crate::{DebugDisplay, ErrorLog};
use alloc::boxed::Box;
#[cfg(feature = "anyhow")]
use core::fmt::Debug;
use log::{debug, error, info, trace, warn, LevelFilter};
#[cfg(feature = "native-dialog")]
use {core::fmt::Display, native_dialog::MessageType};

/**
Pre-defined [ErrorLog][ErrorLog] Using [anyhow::Error]

Its suggested to use [new_anyhow()][crate::ErrorLog::new_anyhow] to load optimised settings.
*/
#[cfg(feature = "anyhow")]
pub type ErrorLogAnyhow<T> = ErrorLog<T, anyhow::Error>;
#[cfg(feature = "anyhow")]
/// Special methods for [ErrorLogAnyhow][crate::ErrorLogAnyhow]
impl<T: std::fmt::Debug> ErrorLog<T, anyhow::Error> {
    /// Creates a new [ErrorLog][crate::ErrorLog] and sets the PrintMode to Debug. Indented for best [anyhow] compatibilty
    pub fn new_anyhow() -> Self {
        let mut out = Self::new();
        out.set_display_mode(crate::FormatMode::Debug);
        out
    }
}

impl<T, E> ErrorLog<T, E> {
    /// print errors using log::error
    pub fn print_fn_log_error(&mut self) -> &mut Self {
        self.print_fn = |e| error!("{e}");
        self
    }
}
