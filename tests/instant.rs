#![cfg(feature = "instant-display")]
use std::num::ParseIntError;

use error_log::ErrorLog;

#[test]
fn instant() {
    let mut err_log = ErrorLog::<i32, ParseIntError>::new();
    err_log.instant_display(true);
    err_log.push_result("abc".parse::<i32>());
    assert_eq!(err_log.entries().len(), 1);
}
