use error_log::{self, ErrorLogBox};
use pretty_assertions::assert_eq;

#[test]
fn boxed() {
    let mut err_log = ErrorLogBox::<i32>::new();
    err_log.push_result_box("abc".parse::<i32>());
    assert_eq!(err_log.entries().len(), 1);
    err_log.push_err_box("custom error");
    assert_eq!(err_log.entries().len(), 2);
    err_log.merge_result_box(dbg!("42".parse::<i32>()));
    assert_eq!(*err_log, Some(42));
}
