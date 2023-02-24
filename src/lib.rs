#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(rustdoc::all)]
/*!
Libary to store errors and log messages and display them later.

*/
mod display;
mod get;
mod macros;
mod manage_errors;
mod messages;
mod presets;
mod traits;

pub use log::LevelFilter;
pub use presets::*;

use std::fmt::{Debug, Display};

/// Type alias for `Vec<Entry<E>>`
pub type Entries<E> = Vec<Entry<E>>;

/**
A Object to store multiple error messages and display them at once

# Operations
x:ErrorLog, E:Error, T:ok value, U:unrestricted type
- `*x`: [ok()][Self::ok]/[ok_mut()][Self::ok_mut]: get (mutable) ok value as [Option]\<T>
- `x += Result<U, E>`: Shorthand for [push_result()][Self::push_result]
- `x += E`: Shorthand for [push_err()][Self::push_err]
- `x *= `[Result]`<T, E>`: Shorthand for [merge_result()][Self::merge_result]
*/
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ErrorLog<T, E = Box<dyn Debug>> {
    display_mode: FormatMode,
    entries: Entries<E>,
    join: Option<String>,
    ok: Option<T>,
    max_level: LevelFilter,
    print_fn: fn(String),
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
/// Entry containg an Error of type `E` or an MessageError
pub enum Entry<E> {
    /// An error of type E
    Error(E),
    /// A logging message
    Message {
        /// Level of message
        level: LevelFilter,
        /// Message Content
        message: String,
    },
}

impl<E: Debug + Display> Entry<E> {
    fn as_string(&self, mode: &FormatMode, max_level: &LevelFilter) -> Option<String> {
        match self {
            Self::Error(err) => Some(match mode {
                FormatMode::Normal => format!("{err}"),
                FormatMode::Debug => format!("{err:?}"),
                FormatMode::PrettyDebug => format!("{err:#?}"),
            }),
            Self::Message {
                message: msg,
                level,
            } => match level <= max_level {
                true => Some(msg.clone()),
                false => None,
            },
        }
    }
}

impl<T, E> Default for ErrorLog<T, E> {
    fn default() -> Self {
        Self {
            ok: None,
            entries: Vec::new(),
            display_mode: FormatMode::default(),
            print_fn: |e| println!("{e}"),
            max_level: LevelFilter::Trace,
            join: None,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// How the error should be printed
pub enum FormatMode {
    /// Uses `{}` (Default)
    #[default]
    Normal,
    /// Uses `{:?}`
    Debug,
    /// Uses `{:#?}`
    PrettyDebug,
}
