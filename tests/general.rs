use error_log::ErrorLog;

struct Foo;

#[test]
/// Confirms that there this create ca be used for Error-Types without any implementations
fn no_traits() {
    ErrorLog::<i32, Foo>::new();
}
