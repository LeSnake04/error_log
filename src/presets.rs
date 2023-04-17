use crate::{format_unix_timestamp, ErrorLog};
#[allow(unused_imports)]
use core::fmt::{Debug, Display};
use log::{debug, error, info, trace, warn, LevelFilter};
#[cfg(feature = "native-dialog")]
use native_dialog::MessageType;
#[cfg(feature = "errors")]
use {crate::DebugDisplay, alloc::boxed::Box};

#[cfg(feature = "anyhow")]
/**
Pre-defined [`ErrorLog`] Using [`anyhow::Error`] as `E`

Its suggested to use [`new_anyhow()`][crate::ErrorLog::new_anyhow] to load optimized settings.
*/
pub type ErrorLogAnyhow<T> = ErrorLog<T, anyhow::Error>;

#[cfg(feature = "anyhow")]
/// Special methods for [`ErrorLogAnyhow`][crate::ErrorLogAnyhow]
impl<T: Debug> ErrorLog<T, anyhow::Error> {
    /// Creates a new [`ErrorLog`][crate::ErrorLog] and sets the [`FormatMode`][crate::FormatMode] to Debug. Indented for best [`anyhow`] compatibility
    pub fn new_anyhow() -> Self {
        let mut out = Self::new();
        out.display_mode(crate::FormatMode::Debug);
        out
    }
}

#[cfg(feature = "errors")]
/**
Pre-defined [`ErrorLog`] using `Box<dyn DebugDisplay>` as `E`

Unlocks additional functions:
- [`merge_result_box()`][Self::merge_result_box]
- [`push_result_box()`][Self::push_result_box]
- [`push_err_box()`][Self::push_err_box]
*/
pub type ErrorLogBox<T> = ErrorLog<T, Box<dyn DebugDisplay>>;

impl<T, E> ErrorLog<T, E> {
    /// Display entries using [`log`] macros
    pub fn display_fn_log(&mut self) -> &mut Self {
        self.display_fn = |level, unix, e| {
            let ts = format_unix_timestamp(unix);
            match level {
                LevelFilter::Off => (),
                LevelFilter::Error => error!("{ts} {e}"),
                LevelFilter::Warn => warn!("{ts} {e}"),
                LevelFilter::Info => info!("{ts} {e}"),
                LevelFilter::Debug => debug!("{ts} {e}"),
                LevelFilter::Trace => trace!("{ts} {e}"),
            }
        };
        self
    }
    /// Display errors using [`println`]
    pub fn display_fn_println(&mut self) -> &mut Self {
        self.display_fn = Self::default().display_fn;
        self
    }
}

#[cfg(feature = "native-dialog")]
impl<T, E: Debug + Display> ErrorLog<T, E> {
    /// Display [`crate::Entries`] using [`native_dialog::MessageDialog`]
    pub fn display_fn_native_dialog(&mut self) -> &mut Self {
        self.display_fn = |lvl, unix_ts, e| {
            if let Err(dialog_err) = native_dialog::MessageDialog::new()
                .set_type(match lvl {
                    LevelFilter::Off => return,
                    LevelFilter::Error => MessageType::Error,
                    LevelFilter::Warn => MessageType::Warning,
                    _ => MessageType::Info,
                })
                .set_title(lvl.as_str())
                .set_text(&format!("{}: {e}", format_unix_timestamp(unix_ts)))
                .show_alert()
            {
                println!("Failed to show MessageDialog: {}", dialog_err)
            }
        };
        self
    }
}
