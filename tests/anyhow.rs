#![cfg(feature = "anyhow")]

use anyhow::Context;
use error_log::ErrorLogAnyhow;

#[test]
fn anyhow() {
    let mut err_log = ErrorLogAnyhow::<String>::new_anyhow();
    err_log += "abc".parse::<i32>().context("Error");
    assert_eq!(err_log.entries().len(), 1);
    assert!(err_log.display_ok().is_none());
}
