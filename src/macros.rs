macro_rules! internal_err {
    ($e:expr) => ({
        $crate::ErrorKind::internal_error(file!(), line!(), $e)
    });
    ($f:tt, $($arg:expr),+) => ({
        internal_err!(format!($f, $($arg),+))
    });
}
