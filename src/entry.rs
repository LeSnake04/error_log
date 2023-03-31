use crate::FormatMode;
use log::LevelFilter;
use {
    alloc::{format, string::String},
    core::fmt::{Debug, Display},
};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
/// Entry containg an Error of type `E` or an MessageError
pub struct Entry<E> {
    /// Content of the entry
    pub content: EntryContent<E>,
    /// Timestamp when the event occured
    pub timestamp: i64,
    pub(crate) instant_display_displayed: bool,
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
    fn new(content: EntryContent<E>) -> Self {
        Self {
            content,
            #[cfg(feature = "std")]
            timestamp: time::OffsetDateTime::now_local()
                .map(|t| t.unix_timestamp())
                .unwrap_or(0),
            #[cfg(not(feature = "std"))]
            timestamp: 0,
            instant_display_displayed: false,
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
    /// Get message as [String]
    pub fn get_message(&self, mode: &FormatMode) -> String {
        match self.get_message_filter(mode, &LevelFilter::Trace) {
            Some(msg) => msg,
            None => unreachable!("No Message should get filtered."),
        }
    }
    /// Get Message as String fitering based on given [LevelFilter]
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
    /// Get level
    /// Reurns [LevelFilter::Error] or the level of the Message
    pub fn get_level(&self) -> LevelFilter {
        match self.content {
            EntryContent::Error(_) => LevelFilter::Error,
            EntryContent::Message { level, .. } => level,
        }
    }
}

impl<E> From<E> for Entry<E> {
    fn from(value: E) -> Self {
        Self::new_error(value)
    }
}
