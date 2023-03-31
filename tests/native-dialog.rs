#![cfg(feature = "native-dialog")]
use std::num::ParseIntError;

use error_log::ErrorLog;

#[test]
fn native_dialog() {
    let mut err_log = ErrorLog::<i32, ParseIntError>::new();
    err_log.display_fn_native_dialog();
    err_log += "abc123".parse::<i32>();
    assert_eq!(err_log.display_ok(), None);
}
