use std::fmt::{Debug, Display};

use crate::{Entry, ErrorLog};

impl<T, E> ErrorLog<T, E> {
    /// Append errors from another instance
    pub fn append_errors<U>(&mut self, other: &mut ErrorLog<U, E>) -> &mut Self {
        self.entries.append(&mut other.entries);
        self
    }
    /// Map instance. You can change error and ok type
    pub fn map<U: Debug, F: Display>(
        self,
        fun: impl FnOnce(Self) -> ErrorLog<U, F>,
    ) -> ErrorLog<U, F> {
        fun(self)
    }

    /// Store [Ok] value from Result or push  [Err] from [Result] to entries  
    pub fn merge_result<U: Into<T>, F: Into<E>>(&mut self, res: Result<U, F>) -> bool {
        let out = res.is_ok();
        match res {
            Ok(o) => self.set_ok(o.into()),
            Err(e) => self.push_err(e.into()),
        };
        out
    }
    /// Create a new Instance
    pub fn new() -> Self {
        Self::default()
    }
    /// Get immutable `ok` value
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
    /// Prepend
    pub fn prepend_entries<U>(&mut self, other: &mut ErrorLog<U, E>) -> &mut Self {
        other.append_errors(self);
        self.append_errors(other);
        self
    }
    /// Get immutable reference to print function
    pub fn print_fn(&self) -> fn(String) {
        self.print_fn
    }
    /// Push error to entries
    pub fn push_err(&mut self, err: impl Into<E>) -> &mut Self {
        self.entries.push(Entry::Error(err.into()));
        self
    }
    /// Push error of result to entries.
    /// Returns Ok value of give result as Option
    pub fn push_result<U, F: Into<E>>(&mut self, res: Result<U, F>) -> Option<U> {
        match res {
            Ok(o) => Some(o),
            Err(err) => {
                self.entries.push(Entry::Error(err.into()));
                None
            }
        }
    }
    /// Set ok value
    pub fn set_ok(&mut self, new: T) -> &mut Self {
        self.ok = Some(new);
        self
    }
    /// Set print function
    pub fn set_print_fn(&mut self, fun: fn(String)) -> &mut Self {
        self.print_fn = fun;
        self
    }
}
