use crate::{Entry::Error, ErrorLog, FormatMode};
use std::fmt::{Debug, Display};

impl<T: Debug, E: Display + Debug> ErrorLog<T, E> {
    fn display_helper(&self) {
        match self.join {
            None => {
                for i in 0..self.entries.len() {
                    if let Some(msg) = self.get_string(i) {
                        (self.print_fn)(msg);
                    }
                }
            }
            Some(ref delimiter) => {
                if let Some(err) = self.join_to_string(delimiter) {
                    (self.print_fn)(err)
                }
            }
        }
    }
    /// display errors and messages and get ok value as [Option]
    pub fn display_ok(self) -> Option<T> {
        self.display_helper();
        self.ok
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
        self.display_helper();
        self.ok.unwrap_or(or)
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
    pub fn format_mode(&self) -> &FormatMode {
        &self.display_mode
    }
    fn get_string(&self, i: usize) -> Option<String> {
        self.entries[i].as_string(self.format_mode(), &self.max_level)
    }
    /// Set weighter to join messages when displaying them.
    pub fn join_on_display(&mut self, delimiter: Option<impl Into<String>>) -> &mut Self {
        self.join = delimiter.map(|d| d.into());
        self
    }
    /// Get String of Combined Error and Messages
    pub fn join_to_string(&self, delimiter: impl Into<String>) -> Option<String> {
        let delimiter = delimiter.into();
        let mut out = String::from("");
        for i in 0..self.entries.len() {
            if let Some(msg) = self.get_string(i) {
                out.push_str(&(msg + &delimiter))
            }
        }
        match out.is_empty() {
            true => None,
            false => Some(out),
        }
    }
    /// Set how the errors should be formatted:
    /// - Normal: `{}` (Default)
    /// - Debug: `{:?}`
    /// - PrettyDebug: `{:#?}`
    pub fn set_display_mode(&mut self, mode: FormatMode) -> &mut Self {
        self.display_mode = mode;
        self
    }
}
impl<T: Debug> ErrorLog<T> {
    /// Puts error in a [Box] and stores it in the Log
    pub fn push_err_box(&mut self, err: impl Debug + 'static) -> &mut Self {
        self.entries.push(Error(Box::new(err)));
        self
    }
    /// If error, put error in a [Box] and stores it.
    /// Returns Ok value of given result as Option
    pub fn push_result_box<U, F: Debug + 'static>(&mut self, res: Result<U, F>) -> Option<U> {
        match res {
            Ok(o) => Some(o),
            Err(err) => {
                self.entries.push(Error(Box::new(err)));
                None
            }
        }
    }
}
