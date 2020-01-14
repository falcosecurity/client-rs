macro_rules! internal_err {
    ($e:expr) => ({
        let kind = $crate::Error::internal_error(
            format!("[{}:{}]: {}", file!(), line!(), $e)
        );
        $crate::Error::from(kind)
    });
    ($f:tt, $($arg:expr),+) => ({
        internal_err!(format!($f, $($arg),+))
    });
}
