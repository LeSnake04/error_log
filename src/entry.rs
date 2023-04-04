use crate::{now, FormatMode};
use alloc::vec::Vec;
use alloc::{format, string::String};
use core::cell::RefCell;
use core::fmt::{Debug, Display};
use log::LevelFilter;

/// Type alias for `Vec<Entry<E>>`
pub type Entries<E> = Vec<Entry<E>>;

/// Additional functions for [`Entries`]
pub trait EntriesExt {
    /// Sets all timestamps to 0
    ///
    /// Useful for removing variation in tests involving [`assert_eq`]/[`assert_ne`]
    fn clear_timestamps(self) -> Self;
}

impl<E> EntriesExt for Entries<E> {
    fn clear_timestamps(mut self) -> Entries<E> {
        for entry in &mut self {
            entry.timestamp = 0;
        }
        self
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
/// Entry containing an Error of type `E` or an log message
pub struct Entry<E> {
    /// Content of the entry
    pub content: EntryContent<E>,
    /// Timestamp when the event occurred
    pub timestamp: i64,
    pub(crate) instant_display_displayed: RefCell<bool>,
}
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
/// The Content of the Entry
pub enum EntryContent<E> {
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

impl<E> Entry<E> {
    /// Get level
    /// Returns [`LevelFilter::Error`] or the level of the Message
    pub fn get_level(&self) -> LevelFilter {
        match self.content {
            EntryContent::Error(_) => LevelFilter::Error,
            EntryContent::Message { level, .. } => level,
        }
    }
    fn new(content: EntryContent<E>) -> Self {
        Self {
            content,
            timestamp: now(),
            instant_display_displayed: RefCell::new(false),
        }
    }
    /// Create entry of type error
    pub fn new_error(err: E) -> Self {
        Self::new(EntryContent::Error(err))
    }
    /// Create Entry of type Error
    pub fn new_message(level: LevelFilter, message: String) -> Self {
        Self::new(EntryContent::Message { level, message })
    }
}
impl<E: Debug + Display> Entry<E> {
    /// Get message as [`String`]
    pub fn get_message(&self, mode: &FormatMode) -> String {
        match self.get_message_filter(mode, &LevelFilter::Trace) {
            Some(msg) => msg,
            None => unreachable!("No Message should get filtered."),
        }
    }
    /// Get Message as String filtering based on given [`LevelFilter`]
    pub fn get_message_filter(&self, mode: &FormatMode, max_level: &LevelFilter) -> Option<String> {
        match &self.content {
            EntryContent::Error(err) => Some(match mode {
                FormatMode::Normal => format!("{err}"),
                FormatMode::Debug => format!("{err:?}"),
                FormatMode::PrettyDebug => format!("{err:#?}"),
            }),
            EntryContent::Message {
                message: msg,
                level,
            } => match level <= max_level {
                true => Some(msg.clone()),
                false => None,
            },
        }
    }
}

impl<E> From<E> for Entry<E> {
    fn from(value: E) -> Self {
        Self::new_error(value)
    }
}
