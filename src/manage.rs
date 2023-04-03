use crate::{instant_display_helper, DebugDisplay, Entry, ErrorLog};
use alloc::{boxed::Box, string::String, vec::Vec};
use core::fmt::{Debug, Display};
use log::LevelFilter;

impl<T, E: Debug + Display> ErrorLog<T, E> {
    /// Appends errors from another instance
    pub fn append_entries<U>(&mut self, other: &mut ErrorLog<U, E>) -> &mut Self {
        instant_display_helper!(self, self);
        self.entries.append(&mut other.entries);
        self
    }
    /// Removes all entries from [Self].
    pub fn clear_entries(&mut self) -> &mut Self {
        self.entries.clear();
        self
    }
    /// Maps instance to change error and ok type
    pub fn map_error_log<U: Debug, F: Display>(
        self,
        fun: impl FnOnce(Self) -> ErrorLog<U, F>,
    ) -> ErrorLog<U, F> {
        fun(self)
    }
    /// Stores [Ok] value from Result or push  [Err] from [Result] to entries  
    pub fn merge_result<U: Into<T>, F: Into<E>>(&mut self, res: Result<U, F>) -> bool {
        let out = res.is_ok();
        match res {
            Ok(o) => {
                self.set_ok(o.into());
            }
            Err(e) => {
                self.push_err(e.into());
                instant_display_helper!(self);
            }
        };
        out
    }
    /// Appends Entries before the Entries of the current Vec
    pub fn prepend_entries<U>(&mut self, other: &mut ErrorLog<U, E>) -> &mut Self {
        let mut entries = Vec::new();
        entries.append(&mut other.entries);
        entries.append(&mut self.entries);
        self.entries = entries;
        instant_display_helper!(self);
        self
    }
    /// Push error to entries
    pub fn push_err(&mut self, err: impl Into<E>) -> &mut Self {
        self.entries.push(Entry::new_error(err.into()));
        instant_display_helper!(self);
        self
    }
    /// Push error of result to entries.
    /// Returns Ok value of give result as Option
    pub fn push_result<U, F: Into<E>>(&mut self, res: Result<U, F>) -> Option<U> {
        match res {
            Ok(o) => Some(o),
            Err(err) => {
                self.entries.push(Entry::new_error(err.into()));
                instant_display_helper!(self);
                None
            }
        }
    }
}
impl<T, E> ErrorLog<T, E> {
    /// Create a new Instance
    pub fn new() -> Self {
        Self::default()
    }
    /// Get immutable reference to `ok` value
    pub fn ok(&self) -> &Option<T> {
        &self.ok
    }
    /// Get owned `ok` value, discarding all errors.
    /// Related: [display_ok()][Self::display_ok]
    pub fn ok_discard_err(self) -> Option<T> {
        self.ok
    }
    /// Get mutable referec eto ok value
    pub fn ok_mut(&mut self) -> &mut Option<T> {
        &mut self.ok
    }
    /// Take ok value
    pub fn ok_take(&mut self) -> Option<T> {
        self.ok.take()
    }
    /// Get display function
    pub fn display_fn(&self) -> fn(LevelFilter, i64, String) {
        self.display_fn
    }
    /// Set ok value. Takes any value that can be converted to String
    pub fn set_ok(&mut self, new: impl Into<T>) -> &mut Self {
        self.ok = Some(new.into());
        self
    }
    /// Removes `ok` value
    pub fn clear_ok(&mut self) -> &mut Self {
        self.ok = None;
        self
    }
    /// Set print function
    pub fn set_print_fn(&mut self, fun: fn(LevelFilter, i64, String)) -> &mut Self {
        self.display_fn = fun;
        self
    }
}

impl<T> ErrorLog<T, Box<dyn DebugDisplay>> {
    /// If the Result is an Ok variant, store ok value.
    /// If the Result is an Err variant, store the Error as Box
    pub fn merge_result_box<U: Into<T>, F: DebugDisplay + 'static>(
        &mut self,
        res: Result<U, F>,
    ) -> bool {
        match res {
            Ok(o) => {
                self.set_ok(o.into());
                true
            }
            Err(e) => {
                self.entries.push(Entry::new_error(Box::new(e)));
                false
            }
        }
    }
    /// Puts error in a [Box] and stores it
    pub fn push_err_box(&mut self, err: impl DebugDisplay + 'static) -> &mut Self {
        self.entries.push(Entry::new_error(Box::new(err)));
        self
    }
    /// If the Result contains an error, put error in a [Box] and store it.
    /// Returns Ok value of given [Result] as [Option]
    pub fn push_result_box<U: Into<T>, F: DebugDisplay + 'static>(
        &mut self,
        res: Result<U, F>,
    ) -> Option<U> {
        match res {
            Ok(o) => Some(o),
            Err(err) => {
                self.entries.push(Entry::new_error(Box::new(err)));
                None
            }
        }
    }
}
