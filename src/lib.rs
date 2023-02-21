pub mod display;
pub mod macros;
mod traits;

use std::fmt::{Debug, Display};

pub use log::LevelFilter;

#[cfg(feature = "anyhow")]

/** Pre-defined [ErrorLog][ErrorLog] Using [anyhow::Error]

Its suggested to use [new_anyhow()][crate::ErrorLog::new_anyhow] to load optimised settings.*/
pub type ErrorLogAnyhow<T> = ErrorLog<T, anyhow::Error>;
#[cfg(feature = "anyhow")]
/// Special methods for [ErrorLogAnyhow][crate::ErrorLogAnyhow]
impl<T: Debug> ErrorLog<T, anyhow::Error> {
    /// Creates a new [ErrorLog][crate::ErrorLog] and sets the PrintMode to Debug. Indented for best [anyhow] compatibilty
    pub fn new_anyhow() -> Self {
        let mut out = Self::new();
        out.set_display_mode(FormatMode::Debug);
        out
    }
}

// #[derive(Derivative)]
// #[derivative(Debug)]
/**
A Object to store multiple error messages and display them at once

# Operations
x:ErrorLog, E:Error, T:ok value, U:unrestricted type
- `*x`: [ok()][Self::ok]/[ok_mut()][Self::ok_mut]: get (mutable) Option\<T> of ok value
- `x += Result<U, E>` [push_result()][Self::push_result]
- `x += E` [push_err()][Self::push_err]
- `x *= `Result<T, E>`
*/
pub struct ErrorLog<T, E = Box<dyn Debug>> {
    display_mode: FormatMode,
    errors: Vec<E>,
    join: Option<String>,
    ok: Option<T>,
    messages: Vec<(LevelFilter, String)>,
    print_fn: Box<dyn Fn(String)>,
}

impl<T, E> Default for ErrorLog<T, E> {
    fn default() -> Self {
        Self {
            ok: None,
            errors: Vec::new(),
            display_mode: FormatMode::default(),
            print_fn: Box::new(|e| println!("{e}")),
            messages: Vec::new(),
            join: None,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// How the error should be printed
pub enum FormatMode {
    #[default]
    /// Uses `{}` (Default)
    Normal,
    /// Uses `{:?}` (Default for [new_anyhow()][ErrorLog::new_anyhow])
    Debug,
    /// Uses `{:#?}`
    PrettyDebug,
}

impl<T, E> ErrorLog<T, E> {
    /// Append errors from another instance
    pub fn append_errors<U>(&mut self, other: &mut ErrorLog<U, E>) -> &mut Self {
        self.errors.append(&mut other.errors);
        self
    }
    /// Get immuatable Vector of errors
    pub fn errors(&self) -> &Vec<E> {
        &self.errors
    }
    /// Get mutable Vector of Errors
    pub fn errors_mut(&mut self) -> &mut Vec<E> {
        &mut self.errors
    }
    /// Get owned Vector of stored errors, Removing them from Instace
    pub fn errors_owned(&mut self) -> Vec<E> {
        let mut out = vec![];
        out.append(&mut self.errors);
        out
    }

    pub fn map<U: Debug, F: Display>(
        self,
        fun: impl FnOnce(Self) -> ErrorLog<U, F>,
    ) -> ErrorLog<U, F> {
        fun(self)
    }

    pub fn map_errors<F>(self, fun: impl FnOnce(Vec<E>) -> Vec<F>) -> ErrorLog<T, F> {
        let mut out = ErrorLog::new();
        *out.errors_mut() = fun(self.errors);
        out
    }
    pub fn merge_result<U: Into<T>, F: Into<E>>(&mut self, res: Result<U, F>) -> bool {
        let out = res.is_ok();
        match res {
            Ok(o) => self.set_ok(o.into()),
            Err(e) => self.push_err(e.into()),
        };
        out
    }
    pub fn new() -> Self {
        Self::default()
    }
    pub fn ok(&self) -> &Option<T> {
        &self.ok
    }
    /// Get Ok value, discarding all errors.
    pub fn ok_discard_err(self) -> Option<T> {
        self.ok
    }
    pub fn ok_mut(&mut self) -> &mut Option<T> {
        &mut self.ok
    }
    pub fn ok_take(&mut self) -> Option<T> {
        self.ok.take()
    }
    pub fn prepend_errors<U>(&mut self, other: &mut ErrorLog<U, E>) -> &mut Self {
        other.append_errors(self);
        self.append_errors(other);
        self
    }
    pub fn print_fn(&mut self, fun: impl Fn(String) + 'static) -> &mut Self {
        self.print_fn = Box::new(fun);
        self
    }
    pub fn push_err(&mut self, err: impl Into<E>) -> &mut Self {
        self.errors.push(err.into());
        self
    }
    pub fn push_result<U, F: Into<E>>(&mut self, res: Result<U, F>) -> Option<U> {
        match res {
            Ok(o) => Some(o),
            Err(err) => {
                self.errors.push(err.into());
                None
            }
        }
    }
    pub fn set_ok(&mut self, new: T) -> &mut Self {
        self.ok = Some(new);
        self
    }
}
