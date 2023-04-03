#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(clippy::all, rustdoc::all, missing_docs)]
/*!
Libary to store errors and log messages and display them later.

*/

extern crate alloc;

mod display;
mod entry;
mod get;
mod helper;
mod macros;
mod manage;
mod messages;
mod presets;
mod traits;

macro_rules! if_std {
    ($($i:item)*) => ($(
        #[cfg(feature = "std")] $i
    )*)
}
macro_rules! if_not_std {
    ($($i:item)*) => ($(
        #[cfg(not(feature = "std"))] $i
    )*)
}

use crate::helper::{format_unix_timestamp, instant_display_helper, now};
use alloc::{fmt::Debug, string::String, vec::Vec};
use core::fmt::Display;
pub use entry::{Entry, EntryContent};
pub use log::LevelFilter;
pub use presets::*;
pub(crate) use {if_not_std, if_std};
if_std! {
    pub use std::{println, print};
}
if_not_std! {
    pub use libc_print::std_name::{println, print};
}

/**
A trait Combining debug and display bounds
*/
pub trait DebugDisplay: Debug + Display {}

impl<T: Debug + Display> DebugDisplay for T {}

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
pub struct ErrorLog<T, E> {
    format_mode: FormatMode,
    entries: Entries<E>,
    instant_display: bool,
    delimiter: String,
    join: bool,
    max_level: LevelFilter,
    max_level_used: LevelFilter,
    ok: Option<T>,
    display_fn: fn(LevelFilter, String),
}

impl<T, E> Default for ErrorLog<T, E> {
    fn default() -> Self {
        Self {
            ok: None,
            entries: Vec::new(),
            format_mode: FormatMode::default(),
            display_fn: |lvl, e| println!("{lvl}: {e}"),
            max_level: LevelFilter::Trace,
            delimiter: "".into(),
            join: false,
            max_level_used: LevelFilter::Off,
            instant_display: false,
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
