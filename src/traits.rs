use crate::{if_std, Entry, ErrorLog};
use alloc::vec::IntoIter;
#[cfg(feature = "helper-traits")]
use core::{
    fmt::Display,
    ops::{AddAssign, Deref, DerefMut, MulAssign},
};
if_std! {
    use {std::process::Termination, core::fmt::Debug};
}

impl<T, E> IntoIterator for ErrorLog<T, E> {
    type Item = Entry<E>;
    type IntoIter = IntoIter<Self::Item>;
    /// Iterates over Error stored.
    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

#[cfg(feature = "helper-traits")]
impl<T, E: Debug + Display> AddAssign<E> for ErrorLog<T, E> {
    /// Make `err_log += ERROR` store error if [`Result`] if an [`Err`].
    ///
    /// Shorthand for [`push_err()`][crate::ErrorLog::push_err]
    fn add_assign(&mut self, rhs: E) {
        self.push_err(rhs);
    }
}

#[cfg(feature = "helper-traits")]
impl<T, U, E: Debug + Display> AddAssign<Result<U, E>> for ErrorLog<T, E> {
    /// Make `err_log += RESULT` store error of [`Result`] if any.
    ///
    /// Shorthand for [`push_result()`][crate::ErrorLog::push_result]
    fn add_assign(&mut self, rhs: Result<U, E>) {
        self.push_result(rhs);
    }
}

#[cfg(feature = "std")]
impl<T, E> Termination for ErrorLog<T, E> {
    fn report(self) -> std::process::ExitCode {
        use std::process::ExitCode;
        match self.ok.is_some() {
            true => ExitCode::SUCCESS,
            false => ExitCode::FAILURE,
        }
    }
}

#[cfg(feature = "helper-traits")]
impl<T, U: Into<T>, E: Debug + Display, F: Into<E>> MulAssign<Result<U, F>> for ErrorLog<T, E> {
    fn mul_assign(&mut self, rhs: Result<U, F>) {
        self.merge_result(rhs);
    }
}

#[cfg(feature = "helper-traits")]
impl<T, U: Into<T>, E> MulAssign<Option<U>> for ErrorLog<T, E> {
    fn mul_assign(&mut self, rhs: Option<U>) {
        if let Some(val) = rhs {
            self.set_ok(val.into());
        };
    }
}

// impl<T: Debug, U: Into<T>, E, F> DivAssign<Result<U, F>> for ErrorLog<T, E> {
//     fn div_assign(&mut self, rhs: Result<U, F>) {
//         self.set_ok()
//     }
// }

#[cfg(feature = "helper-traits")]
/// Get immutable '`ok`' value as [`Option`] by dereferencing
impl<T, E> Deref for ErrorLog<T, E> {
    type Target = Option<T>;
    fn deref(&self) -> &Self::Target {
        self.ok()
    }
}

#[cfg(feature = "helper-traits")]
/// Get mutable '`ok`' value as [`Option`] by dereferencing
impl<T, E> DerefMut for ErrorLog<T, E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.ok_mut()
    }
}
