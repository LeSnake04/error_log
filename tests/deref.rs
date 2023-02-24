use pretty_assertions::assert_eq;

use std::num::ParseIntError;

use error_log::ErrorLog;

fn main() {
    let mut out = ErrorLog::new();
    run(&mut out);
}

fn run(out: &mut ErrorLog<i32, ParseIntError>) {
    assert_eq!(0, out.entries().len());
    *out *= "ab12".parse::<i32>();
    assert_eq!(**out, None);
}
