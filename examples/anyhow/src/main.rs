use anyhow::Context;
use error_log::ErrorLogAnyhow;

fn main() {
    let mut err_log = ErrorLogAnyhow::<String>::new_anyhow();
    err_log += "abc".parse::<i32>().context("Error");
    assert_eq!(err_log.errors().len(), 1);
    assert!(err_log.ok_or_display().is_none());
}
