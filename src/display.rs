use crate::{print, ErrorLog, FormatMode};
use alloc::string::String;
use core::fmt::{Debug, Display};
use log::LevelFilter;

impl<T, E: Display + Debug> ErrorLog<T, E> {
    /// Reset delimiter tu default value.
    pub fn clear_delimiter(&mut self) -> &mut Self {
        self.delimiter = "".into();
        self
    }
    /// Set delimiter. Takes any value that can be converted to String.
    pub fn delimiter(&mut self, delimiter: impl Into<String>) {
        self.delimiter = delimiter.into();
    }
    /// Displays all entries
    pub(crate) fn display_helper(&self) {
        match self.join {
            false => {
                for i in 0..self.entries.len() {
                    if let Some(msg) = self.get_string(i) {
                        (self.display_fn)(self.get_level(i), msg);
                    }
                }
                if !self.delimiter.is_empty() {
                    print!("{}", self.delimiter);
                }
            }
            true => {
                if let Some(err) = self.join_to_string() {
                    (self.display_fn)(LevelFilter::Error, err)
                }
            }
        }
    }
    /// Displays all entries
    pub(crate) fn instant_display_helper(&mut self) {
        match self.join {
            true => {
                for i in 0..self.entries.len() {
                    if let Some(msg) = self.instant_get_string(i) {
                        (self.display_fn)(self.get_level(i), msg);
                    }
                }
            }
            false => {
                if let Some(err) = self.join_to_string() {
                    (self.display_fn)(LevelFilter::Error, err)
                }
            }
        }
    }
    /// Set how the errors should be formatted:
    /// - Normal: `{}` (Default)
    /// - Debug: `{:?}`
    /// - PrettyDebug: `{:#?}`
    pub fn display_mode(&mut self, mode: FormatMode) -> &mut Self {
        self.format_mode = mode;
        self
    }
    /// Get display value or mutable reference to [Option] of ok value
    pub fn display_mut(&mut self) -> Option<&mut T> {
        self.display_helper();
        self.ok.as_mut()
    }
    /// Display entries and take ok value from [Option]
    pub fn display_ok(self) -> Option<T> {
        self.display_helper();
        self.ok
    }
    /// Get display value or reference to [Option] of ok value
    pub fn display_ref(&self) -> Option<&T> {
        self.display_helper();
        self.ok.as_ref()
    }
    /// Display errors and messages and take ok value from [Option]
    pub fn display_take(&mut self) -> Option<T> {
        self.display_helper();
        self.ok.take()
    }
    /// Display entries and get ok value, panicing if no value set.
    ///
    /// Related: [display_ok()][Self::display_ok]
    pub fn display_take_unwrap(&mut self) -> T {
        self.display_take().expect("No Ok value")
    }
    /// Display entries and get ok value, panicing if no value set.
    ///
    /// Related: [display_ok()][Self::display_ok]
    pub fn display_unwrap(self) -> T {
        self.display_ok().expect("No Ok value")
    }
    /// Display entries and get ok value, using given value if no value set
    ///
    /// Its recommeded to use [display_unwrap_or_else()][Self::display_unwrap_or_else] when the alternative value needs to be calculated
    pub fn display_unwrap_or(self, or: T) -> T {
        self.display_ok().unwrap_or(or)
    }
    /// Display entries and get ok value, using default value if no value set
    pub fn display_unwrap_or_default(self) -> T
    where
        T: Default,
    {
        self.display_helper();
        self.ok.unwrap_or_default()
    }
    /// Display entries and get ok value, using value calculated from given closure
    ///
    /// Related: [display_unwrap_or()][Self::display_unwrap_or]
    pub fn display_unwrap_or_else(self, run: impl FnOnce() -> T) -> T {
        self.display_helper();
        self.ok.unwrap_or_else(run)
    }
    /// Get [Format Mode][FormatMode]
    pub fn get_format_mode(&self) -> &FormatMode {
        &self.format_mode
    }
    /// Get weighther the function display incoming entries instantly.
    /// Note: Entries wont get stored when true
    pub fn get_instant_display(&self) -> bool {
        self.instant_display
    }
    /// get level of entry at given index
    fn get_level(&self, index: usize) -> LevelFilter {
        self.entries[index].get_level()
    }
    /// get (error) message of entry at given index
    fn get_string(&self, i: usize) -> Option<String> {
        self.entries[i].get_message_filter(self.get_format_mode(), &self.max_level)
    }
    /// Set weighther the function display incoming entries instantly.
    /// Note: Entries wont get stored when true
    pub fn instant_display(&mut self, val: bool) -> &mut Self {
        self.instant_display = val;
        self
    }
    fn instant_get_string(&mut self, i: usize) -> Option<String> {
        let displayed = &mut self.entries[i].instant_display_displayed;
        if *displayed {
            None?
        }
        *displayed = true;
        self.get_string(i)
    }
    /// Set weighter to join messages into one big String when displaying them.
    pub fn join_on_display(&mut self, join: bool) -> &mut Self {
        self.join = join;
        self
    }
    /// Get String of Combined Error and Messages
    pub fn join_to_string(&self) -> Option<String> {
        let mut out = String::from("");
        for i in 0..self.entries.len() {
            if let Some(msg) = self.get_string(i) {
                out.push_str(&(msg + &self.delimiter))
            }
        }
        match out.is_empty() {
            true => None,
            false => Some(out),
        }
    }
}
