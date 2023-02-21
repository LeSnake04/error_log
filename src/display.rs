use std::fmt::{Debug, Display};

use crate::{ErrorLog, FormatMode};

impl<T: Debug, E: Display + Debug> ErrorLog<T, E> {
    fn as_string(&self, err: &E) -> String {
        match self.display_mode {
            FormatMode::Normal => err.to_string(),
            FormatMode::Debug => format!("{err:?}"),
            FormatMode::PrettyDebug => format!("{err:#?}"),
        }
    }
    fn display(&self) {
        match self.join {
            None => {
                for err in &self.errors {
                    (*self.print_fn)(self.as_string(err));
                }
            }
            Some(ref delimiter) => {
                if let Some(err) = self.join_to_string(delimiter) {
                    (*self.print_fn)(err)
                }
            }
        }
    }
    /// Get [Format Mode][FormatMode]
    pub fn format_mode(&self) -> &FormatMode {
        &self.display_mode
    }
    /// Set
    pub fn join_on_display(&mut self, delimiter: Option<impl Into<String>>) -> &mut Self {
        self.join = delimiter.map(|d| d.into());
        self
    }
    pub fn join_to_string(&self, delimiter: impl Into<String>) -> Option<String> {
        let delimiter = delimiter.into();
        let mut out = String::from("");
        for err in &self.errors {
            out.push_str(&(self.as_string(err) + &delimiter))
        }
        match out.is_empty() {
            true => None,
            false => Some(out),
        }
    }
    pub fn ok_or_display(self) -> Option<T> {
        self.display();
        self.ok
    }
    #[cfg(feature = "log")]
    pub fn print_fn_log_error(&mut self) -> &mut Self {
        self.print_fn = Box::new(|e| log::error!("{e}"));
        self
    }
    /// Set how the errors should be formatted:
    /// - Normal: `{}` (Default)
    /// - Debug: `{:?}` (Default for [new_anyhow()][Self::new_anyhow])
    /// - PrettyDebug: `{:#?}`
    pub fn set_display_mode(&mut self, mode: FormatMode) -> &mut Self {
        self.display_mode = mode;
        self
    }
    pub fn unwrap_or_display(self) -> T {
        self.ok_or_display().expect("No Ok value")
    }
    pub fn unwrap_or_display_and(self, or: T) -> T {
        self.display();
        self.ok.unwrap_or(or)
    }
    pub fn unwrap_or_display_and_default(self) -> T
    where
        T: Default,
    {
        self.display();
        self.ok.unwrap_or_default()
    }
    pub fn unwrap_or_display_and_else(self, run: impl FnOnce() -> T) -> T {
        self.display();
        self.ok.unwrap_or_else(run)
    }
}
impl<T: Debug> ErrorLog<T> {
    /// Puts error in a [Box] and stores it in the Log
    pub fn push_err_box(&mut self, err: impl Debug + 'static) -> &mut Self {
        self.errors.push(Box::new(err));
        self
    }
    pub fn push_result_box<U, F: Debug + 'static>(&mut self, res: Result<U, F>) -> Option<U> {
        match res {
            Ok(o) => Some(o),
            Err(err) => {
                self.errors.push(Box::new(err));
                None
            }
        }
    }
}
